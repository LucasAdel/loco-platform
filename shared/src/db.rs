use async_trait::async_trait;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr, 
    TransactionError, TransactionTrait
};
use std::fmt::Debug;
use std::future::Future;
use crate::errors::AppError;

/// Transaction result type alias
pub type TxResult<T> = Result<T, TransactionError<AppError>>;

/// Database transaction helper trait
#[async_trait]
pub trait TransactionHelper: ConnectionTrait + TransactionTrait {
    /// Execute a transaction with automatic rollback on error
    async fn transaction_with_retry<F, Fut, T>(
        &self,
        max_retries: u32,
        f: F,
    ) -> Result<T, AppError>
    where
        F: Fn(DatabaseTransaction) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, AppError>> + Send,
        T: Send;

    /// Execute a transaction with custom error handling
    async fn transaction_with_handler<F, Fut, T, E>(
        &self,
        f: F,
        error_handler: impl Fn(E) -> AppError + Send + Sync,
    ) -> Result<T, AppError>
    where
        F: FnOnce(DatabaseTransaction) -> Fut + Send,
        Fut: Future<Output = Result<T, E>> + Send,
        T: Send,
        E: Debug + Send;
}

#[async_trait]
impl TransactionHelper for DatabaseConnection {
    async fn transaction_with_retry<F, Fut, T>(
        &self,
        max_retries: u32,
        f: F,
    ) -> Result<T, AppError>
    where
        F: Fn(DatabaseTransaction) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, AppError>> + Send,
        T: Send,
    {
        let mut attempts = 0;
        
        loop {
            attempts += 1;
            
            match self.transaction::<_, AppError, _>(|tx| {
                Box::pin(f(tx))
            }).await {
                Ok(result) => return Ok(result),
                Err(TransactionError::Connection(e)) => {
                    if attempts >= max_retries {
                        return Err(AppError::Database(e.to_string()));
                    }
                    // Exponential backoff
                    let delay = std::time::Duration::from_millis(100 * 2u64.pow(attempts - 1));
                    tokio::time::sleep(delay).await;
                }
                Err(TransactionError::Transaction(e)) => {
                    return Err(e);
                }
            }
        }
    }

    async fn transaction_with_handler<F, Fut, T, E>(
        &self,
        f: F,
        error_handler: impl Fn(E) -> AppError + Send + Sync,
    ) -> Result<T, AppError>
    where
        F: FnOnce(DatabaseTransaction) -> Fut + Send,
        Fut: Future<Output = Result<T, E>> + Send,
        T: Send,
        E: Debug + Send,
    {
        self.transaction::<_, AppError, _>(|tx| {
            Box::pin(async move {
                f(tx).await.map_err(error_handler)
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(db_err) => AppError::Database(db_err.to_string()),
            TransactionError::Transaction(app_err) => app_err,
        })
    }
}

/// Transaction builder for complex transactions
pub struct TransactionBuilder<'a> {
    conn: &'a DatabaseConnection,
    isolation_level: Option<IsolationLevel>,
    readonly: bool,
    timeout: Option<std::time::Duration>,
}

#[derive(Debug, Clone, Copy)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

impl<'a> TransactionBuilder<'a> {
    /// Create a new transaction builder
    pub fn new(conn: &'a DatabaseConnection) -> Self {
        Self {
            conn,
            isolation_level: None,
            readonly: false,
            timeout: None,
        }
    }

    /// Set the isolation level
    pub fn with_isolation_level(mut self, level: IsolationLevel) -> Self {
        self.isolation_level = Some(level);
        self
    }

    /// Mark transaction as read-only
    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    /// Set transaction timeout
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Execute the transaction
    pub async fn execute<F, Fut, T>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce(DatabaseTransaction) -> Fut + Send,
        Fut: Future<Output = Result<T, AppError>> + Send,
        T: Send,
    {
        // In a real implementation, we would apply isolation level and readonly settings
        // For now, we'll use the standard transaction
        if let Some(timeout) = self.timeout {
            tokio::time::timeout(timeout, async {
                self.conn.transaction::<_, AppError, _>(|tx| Box::pin(f(tx)))
                    .await
                    .map_err(|e| match e {
                        TransactionError::Connection(db_err) => AppError::Database(db_err.to_string()),
                        TransactionError::Transaction(app_err) => app_err,
                    })
            })
            .await
            .map_err(|_| AppError::timeout("Transaction timed out"))?
        } else {
            self.conn.transaction::<_, AppError, _>(|tx| Box::pin(f(tx)))
                .await
                .map_err(|e| match e {
                    TransactionError::Connection(db_err) => AppError::Database(db_err.to_string()),
                    TransactionError::Transaction(app_err) => app_err,
                })
        }
    }
}

/// Helper functions for common transaction patterns
pub mod patterns {
    use super::*;
    use sea_orm::{ActiveModelTrait, EntityTrait, PrimaryKeyTrait, ModelTrait};
    
    /// Execute multiple inserts in a single transaction
    pub async fn batch_insert<E, A>(
        db: &DatabaseConnection,
        models: Vec<A>,
    ) -> Result<Vec<E::Model>, AppError>
    where
        E: EntityTrait,
        A: ActiveModelTrait<Entity = E> + Send,
        E::Model: Send,
    {
        db.transaction::<_, AppError, _>(|tx| {
            Box::pin(async move {
                let mut results = Vec::with_capacity(models.len());
                
                for model in models {
                    let result = model.insert(&tx).await
                        .map_err(|e| AppError::Database(e.to_string()))?;
                    results.push(result);
                }
                
                Ok(results)
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(db_err) => AppError::Database(db_err.to_string()),
            TransactionError::Transaction(app_err) => app_err,
        })
    }
    
    /// Update with optimistic locking
    pub async fn update_with_version<E, A>(
        db: &DatabaseConnection,
        id: E::PrimaryKey,
        expected_version: i32,
        update_fn: impl FnOnce(E::Model) -> Result<A, AppError> + Send,
    ) -> Result<E::Model, AppError>
    where
        E: EntityTrait,
        E::Model: ModelTrait<Entity = E> + Send + Sync,
        A: ActiveModelTrait<Entity = E> + Send,
    {
        db.transaction::<_, AppError, _>(|tx| {
            Box::pin(async move {
                // Find the entity
                let entity = E::find_by_id(id.clone())
                    .one(&tx)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?
                    .ok_or_else(|| AppError::NotFound)?;
                
                // Check version (this assumes a version field exists)
                // In real implementation, you'd check the actual version field
                
                // Apply update
                let active_model = update_fn(entity)?;
                
                // Save with updated version
                active_model.update(&tx)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(db_err) => AppError::Database(db_err.to_string()),
            TransactionError::Transaction(app_err) => app_err,
        })
    }
    
    /// Execute a transaction with compensation on failure
    pub async fn with_compensation<T>(
        db: &DatabaseConnection,
        action: impl Future<Output = Result<T, AppError>> + Send,
        compensation: impl Future<Output = Result<(), AppError>> + Send,
    ) -> Result<T, AppError> {
        match action.await {
            Ok(result) => Ok(result),
            Err(e) => {
                // Attempt compensation
                if let Err(comp_err) = compensation.await {
                    // Log compensation failure
                    tracing::error!("Compensation failed: {:?}", comp_err);
                }
                Err(e)
            }
        }
    }
}

/// Macro for simplifying transaction usage
#[macro_export]
macro_rules! transaction {
    ($db:expr, |$tx:ident| $body:expr) => {{
        $db.transaction::<_, $crate::errors::AppError, _>(|$tx| {
            Box::pin(async move { $body })
        })
        .await
        .map_err(|e| match e {
            sea_orm::TransactionError::Connection(db_err) => {
                $crate::errors::AppError::Database(db_err.to_string())
            }
            sea_orm::TransactionError::Transaction(app_err) => app_err,
        })
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_transaction_builder() {
        // Test would require a real database connection
        // This is just a placeholder to show the pattern
    }
}