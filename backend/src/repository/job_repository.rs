use async_trait::async_trait;
use sea_orm::{
    entity::*,
    query::*,
    DatabaseConnection, DbErr, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::entities::{job, user};
use shared::types::{JobFilters, JobStatus, JobType, AustralianState};
use super::{BaseRepository, PaginationParams, PaginatedResult};

pub struct JobRepository;

impl JobRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BaseRepository<job::ActiveModel, job::Model> for JobRepository {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<job::Model>, DbErr> {
        job::Entity::find_by_id(id)
            .filter(job::Column::DeletedAt.is_null()) // Exclude soft-deleted
            .one(db)
            .await
    }

    async fn create(&self, db: &DatabaseConnection, model: job::ActiveModel) -> Result<job::Model, DbErr> {
        model.insert(db).await
    }

    async fn update(&self, db: &DatabaseConnection, model: job::ActiveModel) -> Result<job::Model, DbErr> {
        model.update(db).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: Uuid) -> Result<(), DbErr> {
        // Soft delete by setting deleted_at
        let mut job: job::ActiveModel = job::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Job not found".to_string()))?
            .into();
        
        job.deleted_at = Set(Some(Utc::now().into()));
        job.update(db).await?;
        
        Ok(())
    }
}

impl JobRepository {
    /// Find all active jobs with pagination
    pub async fn find_active_jobs(
        &self,
        db: &DatabaseConnection,
        pagination: PaginationParams,
    ) -> Result<PaginatedResult<job::Model>, DbErr> {
        let select = job::Entity::find()
            .filter(job::Column::Status.eq("Active"))
            .filter(job::Column::DeletedAt.is_null())
            .order_by_desc(job::Column::CreatedAt);

        let paginator = select.paginate(db, pagination.page_size);
        let total_count = paginator.num_items().await?;
        let items = paginator.fetch_page(pagination.page - 1).await?;

        Ok(PaginatedResult::new(items, total_count, pagination))
    }

    /// Search jobs with filters
    pub async fn search_jobs(
        &self,
        db: &DatabaseConnection,
        filters: JobFilters,
        pagination: PaginationParams,
    ) -> Result<PaginatedResult<job::Model>, DbErr> {
        let mut select = job::Entity::find()
            .filter(job::Column::DeletedAt.is_null());

        // Apply filters
        if let Some(job_type) = filters.job_type {
            let db_job_type: job::JobTypeDb = job_type.into();
            select = select.filter(job::Column::JobType.eq(db_job_type));
        }

        if let Some(min_rate) = filters.min_rate {
            select = select.filter(job::Column::HourlyRate.gte(rust_decimal::Decimal::from_f64_retain(min_rate).unwrap_or_default()));
        }

        if let Some(max_rate) = filters.max_rate {
            select = select.filter(job::Column::HourlyRate.lte(rust_decimal::Decimal::from_f64_retain(max_rate).unwrap_or_default()));
        }

        if let Some(suburb) = filters.suburb {
            select = select.filter(job::Column::Suburb.contains(&suburb));
        }

        if let Some(state) = filters.state {
            select = select.filter(job::Column::State.eq(state));
        }

        if let Some(is_urgent) = filters.is_urgent {
            select = select.filter(job::Column::IsUrgent.eq(is_urgent));
        }

        if let Some(start_date) = filters.start_date {
            select = select.filter(job::Column::StartDate.gte(start_date));
        }

        if let Some(end_date) = filters.end_date {
            select = select.filter(job::Column::EndDate.lte(end_date));
        }

        // Order by urgency and creation date
        select = select
            .order_by_desc(job::Column::IsUrgent)
            .order_by_desc(job::Column::CreatedAt);

        let paginator = select.paginate(db, pagination.page_size);
        let total_count = paginator.num_items().await?;
        let items = paginator.fetch_page(pagination.page - 1).await?;

        Ok(PaginatedResult::new(items, total_count, pagination))
    }

    /// Find jobs near coordinates (requires latitude and longitude)
    pub async fn find_jobs_near_location(
        &self,
        db: &DatabaseConnection,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
        pagination: PaginationParams,
    ) -> Result<PaginatedResult<job::Model>, DbErr> {
        // This is a simplified version. In production, you'd use PostGIS or similar
        // For now, we'll fetch all jobs with coordinates and filter in Rust
        let jobs = job::Entity::find()
            .filter(job::Column::Status.eq("Active"))
            .filter(job::Column::DeletedAt.is_null())
            .filter(job::Column::Latitude.is_not_null())
            .filter(job::Column::Longitude.is_not_null())
            .all(db)
            .await?;

        let mut nearby_jobs: Vec<job::Model> = jobs
            .into_iter()
            .filter_map(|job| {
                if let (Some(job_lat), Some(job_lng)) = (job.latitude, job.longitude) {
                    let distance = calculate_distance(latitude, longitude, job_lat, job_lng);
                    if distance <= radius_km {
                        Some(job)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Sort by distance (closest first)
        nearby_jobs.sort_by(|a, b| {
            let dist_a = calculate_distance(latitude, longitude, a.latitude.unwrap(), a.longitude.unwrap());
            let dist_b = calculate_distance(latitude, longitude, b.latitude.unwrap(), b.longitude.unwrap());
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        let total_count = nearby_jobs.len() as u64;
        let start_idx = pagination.offset() as usize;
        let end_idx = (start_idx + pagination.page_size as usize).min(nearby_jobs.len());
        
        let page_items = if start_idx < nearby_jobs.len() {
            nearby_jobs[start_idx..end_idx].to_vec()
        } else {
            vec![]
        };

        Ok(PaginatedResult::new(page_items, total_count, pagination))
    }

    /// Find jobs by user (employer)
    pub async fn find_jobs_by_user(
        &self,
        db: &DatabaseConnection,
        user_id: Uuid,
        pagination: PaginationParams,
    ) -> Result<PaginatedResult<job::Model>, DbErr> {
        let select = job::Entity::find()
            .filter(job::Column::CreatedBy.eq(user_id))
            .filter(job::Column::DeletedAt.is_null())
            .order_by_desc(job::Column::CreatedAt);

        let paginator = select.paginate(db, pagination.page_size);
        let total_count = paginator.num_items().await?;
        let items = paginator.fetch_page(pagination.page - 1).await?;

        Ok(PaginatedResult::new(items, total_count, pagination))
    }

    /// Get job statistics for admin dashboard
    pub async fn get_job_statistics(&self, db: &DatabaseConnection) -> Result<JobStatistics, DbErr> {
        let total_jobs = job::Entity::find()
            .filter(job::Column::DeletedAt.is_null())
            .count(db)
            .await?;

        let active_jobs = job::Entity::find()
            .filter(job::Column::Status.eq("Active"))
            .filter(job::Column::DeletedAt.is_null())
            .count(db)
            .await?;

        let urgent_jobs = job::Entity::find()
            .filter(job::Column::IsUrgent.eq(true))
            .filter(job::Column::Status.eq("Active"))
            .filter(job::Column::DeletedAt.is_null())
            .count(db)
            .await?;

        let filled_jobs = job::Entity::find()
            .filter(job::Column::Status.eq("Filled"))
            .filter(job::Column::DeletedAt.is_null())
            .count(db)
            .await?;

        Ok(JobStatistics {
            total_jobs,
            active_jobs,
            urgent_jobs,
            filled_jobs,
        })
    }

    /// Increment view count for a job
    pub async fn increment_view_count(&self, db: &DatabaseConnection, job_id: Uuid) -> Result<(), DbErr> {
        let job = job::Entity::find_by_id(job_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Job not found".to_string()))?;

        let mut job: job::ActiveModel = job.into();
        job.view_count = Set(job.view_count.unwrap() + 1);
        job.update(db).await?;

        Ok(())
    }

    /// Find jobs expiring soon (within next 7 days)
    pub async fn find_expiring_jobs(&self, db: &DatabaseConnection) -> Result<Vec<job::Model>, DbErr> {
        let seven_days_from_now = Utc::now() + chrono::Duration::days(7);
        
        job::Entity::find()
            .filter(job::Column::Status.eq("Active"))
            .filter(job::Column::DeletedAt.is_null())
            .filter(job::Column::EndDate.lte(seven_days_from_now))
            .order_by_asc(job::Column::EndDate)
            .all(db)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct JobStatistics {
    pub total_jobs: u64,
    pub active_jobs: u64,
    pub urgent_jobs: u64,
    pub filled_jobs: u64,
}

/// Calculate distance between two points using Haversine formula
fn calculate_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let earth_radius = 6371.0; // Earth's radius in kilometres
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) +
            lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    earth_radius * c
}