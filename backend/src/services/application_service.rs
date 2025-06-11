use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use crate::{
    entities::application::{self, ApplicationStatus, Entity as Application},
    repository::{ApplicationRepository, BaseRepository},
    AppError, AppState,
};
use shared::types::{CreateApplicationRequest, UpdateApplicationRequest, ApplicationStatus as SharedApplicationStatus};
use sea_orm::{DatabaseConnection, EntityTrait, Set, QueryFilter, ColumnTrait};

// Convert from shared ApplicationStatus to entity ApplicationStatus
fn convert_status(status: SharedApplicationStatus) -> ApplicationStatus {
    match status {
        SharedApplicationStatus::Pending => ApplicationStatus::Pending,
        SharedApplicationStatus::Reviewing => ApplicationStatus::Reviewing,
        SharedApplicationStatus::Shortlisted => ApplicationStatus::Shortlisted,
        SharedApplicationStatus::Interviewed => ApplicationStatus::Interviewed,
        SharedApplicationStatus::Offered => ApplicationStatus::Offered,
        SharedApplicationStatus::Accepted => ApplicationStatus::Accepted,
        SharedApplicationStatus::Rejected => ApplicationStatus::Rejected,
        SharedApplicationStatus::Withdrawn => ApplicationStatus::Withdrawn,
    }
}

#[derive(Clone)]
pub struct ApplicationService {
    db: DatabaseConnection,
}

impl ApplicationService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_application(
        &self,
        request: CreateApplicationRequest,
        user_id: Uuid,
    ) -> Result<application::Model, AppError> {
        let new_application = application::ActiveModel {
            job_id: Set(request.job_id),
            user_id: Set(user_id),
            cover_letter: Set(request.cover_letter),
            status: Set(ApplicationStatus::Pending),
            applied_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        };

        let repo = ApplicationRepository::new();
        let application = repo
            .create(&self.db, new_application)
            .await
            .map_err(|e| AppError::Database(format!("Failed to create application: {}", e)))?;

        Ok(application)
    }

    pub async fn get_application_by_id(
        &self,
        id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<application::Model, AppError> {
        let repo = ApplicationRepository::new();
        let application = repo
            .find_by_id(&self.db, id)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?
            .ok_or(AppError::NotFound)?;

        // If user_id is provided, ensure they can only access their own applications
        if let Some(uid) = user_id {
            if application.user_id != uid {
                return Err(AppError::NotFound);
            }
        }

        Ok(application)
    }

    pub async fn update_application(
        &self,
        id: Uuid,
        request: UpdateApplicationRequest,
        user_id: Option<Uuid>,
    ) -> Result<application::Model, AppError> {
        let existing = self.get_application_by_id(id, user_id).await?;

        let mut application: application::ActiveModel = existing.into();
        
        if let Some(cover_letter) = request.cover_letter {
            application.cover_letter = Set(Some(cover_letter));
        }
        
        if let Some(status) = request.status {
            let entity_status = convert_status(status);
            
            // Update reviewed_at timestamp when status changes
            if matches!(entity_status, ApplicationStatus::Reviewing) {
                application.reviewed_at = Set(Some(Utc::now().into()));
            }
            
            application.status = Set(entity_status);
        }

        application.updated_at = Set(Utc::now().into());

        let repo = ApplicationRepository::new();
        let updated = repo
            .update(&self.db, application)
            .await
            .map_err(|e| AppError::Database(format!("Failed to update application: {}", e)))?;

        Ok(updated)
    }

    pub async fn delete_application(
        &self,
        id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<(), AppError> {
        let existing = self.get_application_by_id(id, user_id).await?;

        let repo = ApplicationRepository::new();
        repo.delete(&self.db, existing.id)
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete application: {}", e)))?;

        Ok(())
    }

    pub async fn get_applications_for_job(
        &self,
        job_id: Uuid,
        _pagination: crate::repository::PaginationParams,
        _user_id: Uuid,
    ) -> Result<Vec<application::Model>, AppError> {
        let applications = Application::find()
            .filter(application::Column::JobId.eq(job_id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?;
        
        Ok(applications)
    }

    pub async fn get_applications_by_user(
        &self,
        user_id: Uuid,
        _pagination: crate::repository::PaginationParams,
    ) -> Result<Vec<application::Model>, AppError> {
        let applications = Application::find()
            .filter(application::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?;
        
        Ok(applications)
    }

    pub async fn list_applications(
        &self,
        _filters: crate::handlers::applications::ApplicationFilters,
        _pagination: crate::repository::PaginationParams,
        user_id: Uuid,
    ) -> Result<Vec<application::Model>, AppError> {
        // For now, just return applications for the user
        // TODO: Implement proper filtering and pagination
        let applications = Application::find()
            .filter(application::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?;
        
        Ok(applications)
    }

    pub async fn get_application_statistics(
        &self,
        user_id: Option<Uuid>,
    ) -> Result<HashMap<String, i64>, AppError> {
        let mut query = Application::find();
        
        if let Some(uid) = user_id {
            query = query.filter(application::Column::UserId.eq(uid));
        }
        
        let applications = query
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?;
        
        let mut stats = HashMap::new();
        stats.insert("total".to_string(), applications.len() as i64);
        
        // Count by status
        let mut status_counts: HashMap<ApplicationStatus, i64> = HashMap::new();
        for app in applications {
            *status_counts.entry(app.status).or_insert(0) += 1;
        }
        
        for (status, count) in status_counts {
            stats.insert(format!("{:?}", status).to_lowercase(), count);
        }
        
        Ok(stats)
    }

    pub async fn withdraw_application(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<application::Model, AppError> {
        let request = UpdateApplicationRequest {
            status: Some(SharedApplicationStatus::Withdrawn),
            cover_letter: None,
        };
        
        self.update_application(id, request, Some(user_id)).await
    }

    pub async fn update_application_status(
        &self,
        id: Uuid,
        status: SharedApplicationStatus,
        employer_user_id: Option<Uuid>,
    ) -> Result<application::Model, AppError> {
        let request = UpdateApplicationRequest {
            status: Some(status),
            cover_letter: None,
        };
        
        self.update_application(id, request, employer_user_id).await
    }
}

impl From<&AppState> for ApplicationService {
    fn from(state: &AppState) -> Self {
        Self::new(state.db.clone())
    }
}