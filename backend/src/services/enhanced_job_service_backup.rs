use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;
use crate::{
    entities::job::{self, Entity as Job},
    repository::{BaseRepository, JobRepository, PaginationParams},
    AppError, AppState,
};
use shared::types::{JobFilters, CreateJobRequest};
use sea_orm::{DatabaseConnection, EntityTrait, Set, QueryFilter, ColumnTrait, QueryOrder, PaginatorTrait};

#[derive(Clone)]
pub struct EnhancedJobService {
    db: DatabaseConnection,
}

impl EnhancedJobService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Create a new job posting
    pub async fn create_job(
        &self,
        request: CreateJobRequest,
        user_id: Uuid,
    ) -> Result<job::Model, AppError> {
        // Convert shared types to entity types
        let entity_job_type = match request.job_type {
            SharedJobType::Intern => EntityJobType::Intern,
            SharedJobType::Student => EntityJobType::Student,
            SharedJobType::Pharmacist => EntityJobType::Pharmacist,
            SharedJobType::PharmacyAssistant => EntityJobType::PharmacyAssistant,
            SharedJobType::PharmacyTechnician => EntityJobType::PharmacyTechnician,
        };

        let new_job = job::ActiveModel {
            user_id: Set(user_id),
            title: Set(request.title),
            description: Set(request.description),
            pharmacy_name: Set(request.pharmacy_name),
            hourly_rate: Set(request.hourly_rate),
            address: Set(request.address),
            suburb: Set(request.suburb),
            postcode: Set(request.postcode),
            state: Set(request.state),
            latitude: Set(request.latitude),
            longitude: Set(request.longitude),
            start_date: Set(request.start_date.naive_utc()),
            end_date: Set(request.end_date.naive_utc()),
            start_time: Set(request.start_time),
            end_time: Set(request.end_time),
            job_type: Set(entity_job_type),
            is_urgent: Set(request.is_urgent),
            status: Set(EntityJobStatus::Active),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let job = new_job
            .insert(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Failed to create job: {}", e)))?;

        Ok(job)
    }

    /// Get job by ID
    pub async fn get_job_by_id(
        &self,
        id: Uuid,
    ) -> Result<job::Model, AppError> {
        Job::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Database error: {}", e)))?
            .ok_or(AppError::NotFound)
    }

    /// List jobs with filtering and pagination
    pub async fn list_jobs(
        &self,
        filters: JobFilters,
        pagination: PaginationParams,
    ) -> Result<(Vec<job::Model>, u64), AppError> {
        let mut query = Job::find();

        // Apply filters
        if let Some(job_type) = filters.job_type {
            let entity_type = match job_type {
                SharedJobType::Intern => EntityJobType::Intern,
                SharedJobType::Student => EntityJobType::Student,
                SharedJobType::Pharmacist => EntityJobType::Pharmacist,
                SharedJobType::PharmacyAssistant => EntityJobType::PharmacyAssistant,
                SharedJobType::PharmacyTechnician => EntityJobType::PharmacyTechnician,
            };
            query = query.filter(job::Column::JobType.eq(entity_type));
        }

        if let Some(min_rate) = filters.min_rate {
            query = query.filter(job::Column::HourlyRate.gte(min_rate));
        }

        if let Some(max_rate) = filters.max_rate {
            query = query.filter(job::Column::HourlyRate.lte(max_rate));
        }

        if let Some(suburb) = filters.suburb {
            query = query.filter(job::Column::Suburb.contains(&suburb));
        }

        if let Some(state) = filters.state {
            query = query.filter(job::Column::State.eq(&state));
        }

        if let Some(is_urgent) = filters.is_urgent {
            query = query.filter(job::Column::IsUrgent.eq(is_urgent));
        }

        if let Some(start_date) = filters.start_date {
            query = query.filter(job::Column::StartDate.gte(start_date.naive_utc()));
        }

        if let Some(end_date) = filters.end_date {
            query = query.filter(job::Column::EndDate.lte(end_date.naive_utc()));
        }

        // Only show active jobs by default
        query = query.filter(job::Column::Status.eq(EntityJobStatus::Active));

        // Apply ordering - most recent first
        query = query.order_by_desc(job::Column::CreatedAt);

        // Get total count for pagination
        let total = query.clone().count(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Count query failed: {}", e)))?;

        // Apply pagination
        let jobs = query
            .offset(pagination.offset())
            .limit(pagination.page_size())
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("List query failed: {}", e)))?;

        Ok((jobs, total))
    }

    /// Update job status
    pub async fn update_job_status(
        &self,
        id: Uuid,
        status: SharedJobStatus,
        user_id: Uuid,
    ) -> Result<job::Model, AppError> {
        let existing = self.get_job_by_id(id).await?;

        // Verify ownership
        if existing.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        let entity_status = match status {
            SharedJobStatus::Active => EntityJobStatus::Active,
            SharedJobStatus::Closed => EntityJobStatus::Closed,
            SharedJobStatus::Draft => EntityJobStatus::Draft,
            SharedJobStatus::Filled => EntityJobStatus::Filled,
            SharedJobStatus::Cancelled => EntityJobStatus::Cancelled,
            SharedJobStatus::Expired => EntityJobStatus::Expired,
        };

        let mut job: job::ActiveModel = existing.into();
        job.status = Set(entity_status);
        job.updated_at = Set(Utc::now().naive_utc());

        let updated = job
            .update(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Failed to update job: {}", e)))?;

        Ok(updated)
    }

    /// Delete job
    pub async fn delete_job(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let existing = self.get_job_by_id(id).await?;

        // Verify ownership
        if existing.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        Job::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Failed to delete job: {}", e)))?;

        Ok(())
    }

    /// Get jobs by user
    pub async fn get_jobs_by_user(
        &self,
        user_id: Uuid,
        pagination: PaginationParams,
    ) -> Result<(Vec<job::Model>, u64), AppError> {
        let query = Job::find()
            .filter(job::Column::UserId.eq(user_id))
            .order_by_desc(job::Column::CreatedAt);

        let total = query.clone().count(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Count query failed: {}", e)))?;

        let jobs = query
            .offset(pagination.offset())
            .limit(pagination.page_size())
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("List query failed: {}", e)))?;

        Ok((jobs, total))
    }

    /// Search jobs by text
    pub async fn search_jobs(
        &self,
        query: &str,
        filters: JobFilters,
        pagination: PaginationParams,
    ) -> Result<(Vec<job::Model>, u64), AppError> {
        let mut db_query = Job::find();

        // Text search across title, description, and pharmacy name
        let search_condition = job::Column::Title.contains(query)
            .or(job::Column::Description.contains(query))
            .or(job::Column::PharmacyName.contains(query));

        db_query = db_query.filter(search_condition);

        // Apply additional filters
        if let Some(job_type) = filters.job_type {
            let entity_type = match job_type {
                SharedJobType::Intern => EntityJobType::Intern,
                SharedJobType::Student => EntityJobType::Student,
                SharedJobType::Pharmacist => EntityJobType::Pharmacist,
                SharedJobType::PharmacyAssistant => EntityJobType::PharmacyAssistant,
                SharedJobType::PharmacyTechnician => EntityJobType::PharmacyTechnician,
            };
            db_query = db_query.filter(job::Column::JobType.eq(entity_type));
        }

        // Only show active jobs
        db_query = db_query.filter(job::Column::Status.eq(EntityJobStatus::Active));

        // Order by relevance (for now, just by creation date)
        db_query = db_query.order_by_desc(job::Column::CreatedAt);

        let total = db_query.clone().count(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Search count failed: {}", e)))?;

        let jobs = db_query
            .offset(pagination.offset())
            .limit(pagination.page_size())
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Search query failed: {}", e)))?;

        Ok((jobs, total))
    }

    /// Get job statistics
    pub async fn get_job_statistics(
        &self,
        user_id: Option<Uuid>,
    ) -> Result<HashMap<String, i64>, AppError> {
        let mut query = Job::find();

        if let Some(uid) = user_id {
            query = query.filter(job::Column::UserId.eq(uid));
        }

        let jobs = query
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Statistics query failed: {}", e)))?;

        let mut stats = HashMap::new();
        stats.insert("total".to_string(), jobs.len() as i64);

        // Count by status
        let mut status_counts: HashMap<EntityJobStatus, i64> = HashMap::new();
        for job in jobs {
            *status_counts.entry(job.status).or_insert(0) += 1;
        }

        for (status, count) in status_counts {
            stats.insert(format!("{:?}", status).to_lowercase(), count);
        }

        Ok(stats)
    }

    /// Find jobs near a location
    pub async fn find_jobs_near_location(
        &self,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
        filters: JobFilters,
        pagination: PaginationParams,
    ) -> Result<(Vec<job::Model>, u64), AppError> {
        // For now, get all jobs and filter by distance
        // TODO: Implement proper spatial indexing with PostGIS
        
        let mut query = Job::find()
            .filter(job::Column::Latitude.is_not_null())
            .filter(job::Column::Longitude.is_not_null())
            .filter(job::Column::Status.eq(EntityJobStatus::Active));

        // Apply other filters
        if let Some(job_type) = filters.job_type {
            let entity_type = match job_type {
                SharedJobType::Intern => EntityJobType::Intern,
                SharedJobType::Student => EntityJobType::Student,
                SharedJobType::Pharmacist => EntityJobType::Pharmacist,
                SharedJobType::PharmacyAssistant => EntityJobType::PharmacyAssistant,
                SharedJobType::PharmacyTechnician => EntityJobType::PharmacyTechnician,
            };
            query = query.filter(job::Column::JobType.eq(entity_type));
        }

        let all_jobs = query
            .all(&self.db)
            .await
            .map_err(|e| AppError::Database(format!("Location query failed: {}", e)))?;

        // Filter by distance
        let nearby_jobs: Vec<job::Model> = all_jobs
            .into_iter()
            .filter(|job| {
                if let (Some(job_lat), Some(job_lng)) = (job.latitude, job.longitude) {
                    let distance = Self::calculate_distance(latitude, longitude, job_lat, job_lng);
                    distance <= radius_km
                } else {
                    false
                }
            })
            .collect();

        let total = nearby_jobs.len() as u64;

        // Apply pagination
        let start = pagination.offset() as usize;
        let end = (start + pagination.page_size() as usize).min(nearby_jobs.len());
        let paginated_jobs = nearby_jobs.into_iter().skip(start).take(end - start).collect();

        Ok((paginated_jobs, total))
    }

    /// Calculate distance between two points using Haversine formula
    fn calculate_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let dlat = (lat2 - lat1).to_radians();
        let dlng = (lng2 - lng1).to_radians();

        let a = (dlat / 2.0).sin().powi(2)
            + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlng / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }
}

impl From<&AppState> for EnhancedJobService {
    fn from(state: &AppState) -> Self {
        Self::new(state.db.clone())
    }
}