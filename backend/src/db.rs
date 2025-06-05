use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DbErr,
};
use std::time::Duration;
use tracing::log;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub sqlx_logging: bool,
    pub sqlx_logging_level: log::LevelFilter,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/loco_platform".to_string()),
            max_connections: 100,
            min_connections: 5,
            connect_timeout: Duration::from_secs(8),
            acquire_timeout: Duration::from_secs(8),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(1800),
            sqlx_logging: true,
            sqlx_logging_level: log::LevelFilter::Debug,
        }
    }
}

impl DatabaseConfig {
    /// Create from environment variables
    pub fn from_env() -> Result<Self, String> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL not found in environment")?;
        
        Ok(Self {
            url: database_url,
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            min_connections: std::env::var("DB_MIN_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            ..Default::default()
        })
    }
}

/// Create database connection pool
pub async fn create_pool(config: DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(config.url);
    
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(config.connect_timeout)
        .acquire_timeout(config.acquire_timeout)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .sqlx_logging(config.sqlx_logging)
        .sqlx_logging_level(config.sqlx_logging_level);
    
    Database::connect(opt).await
}

/// Health check for database connection
pub async fn health_check(db: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    
    let query = match db.get_database_backend() {
        DbBackend::Postgres => "SELECT 1",
        DbBackend::MySql => "SELECT 1",
        DbBackend::Sqlite => "SELECT 1",
    };
    
    db.execute(Statement::from_string(
        db.get_database_backend(),
        query.to_owned(),
    ))
    .await?;
    
    Ok(())
}

/// Multi-tenant database utilities
pub mod multi_tenant {
    use super::*;
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    use uuid::Uuid;
    
    /// Set the current tenant context for the database session
    pub async fn set_tenant_context(
        db: &DatabaseConnection,
        tenant_id: Uuid,
    ) -> Result<(), DbErr> {
        let query = format!("SELECT set_config('app.current_tenant', '{}', false)", tenant_id);
        
        db.execute(Statement::from_string(
            db.get_database_backend(),
            query,
        ))
        .await?;
        
        Ok(())
    }
    
    /// Get the current tenant context from the database session
    pub async fn get_tenant_context(db: &DatabaseConnection) -> Result<Option<Uuid>, DbErr> {
        let query = "SELECT current_setting('app.current_tenant', true)";
        
        let result = db
            .query_one(Statement::from_string(
                db.get_database_backend(),
                query.to_owned(),
            ))
            .await?;
        
        if let Some(row) = result {
            if let Ok(Some(tenant_str)) = row.try_get::<Option<String>>("", "current_setting") {
                return Ok(tenant_str.parse().ok());
            }
        }
        
        Ok(None)
    }
}

/// Database initialization
pub async fn initialize_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    
    // Enable UUID extension for PostgreSQL
    if matches!(db.get_database_backend(), DbBackend::Postgres) {
        db.execute(Statement::from_string(
            db.get_database_backend(),
            "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"".to_owned(),
        ))
        .await?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_config() {
        let config = DatabaseConfig::default();
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.min_connections, 5);
    }
}