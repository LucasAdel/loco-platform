use uuid::Uuid;
use shared::types::{Job as SharedJob, JobFilters, CreateJobRequest, JobType, JobStatus, JobId, UserId, Postcode, AustralianState};
use shared::errors::AppError;

pub struct JobService;

impl JobService {
    /// Get all jobs with optional filtering and pagination (Demo mode)
    pub async fn list_jobs(
        _filters: JobFilters,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<(Vec<SharedJob>, u64), shared::errors::AppError> {
        let _page = page.unwrap_or(1);
        let _limit = limit.unwrap_or(20);
        
        // Demo mode: Return sample data
        // TODO: Replace with actual database implementation
        let jobs = Self::get_sample_jobs();
        let total = jobs.len() as u64;
        
        Ok((jobs, total))
    }
    
    /// Get a specific job by ID (Demo mode)
    pub async fn get_job(
        job_id: Uuid,
    ) -> Result<Option<SharedJob>, shared::errors::AppError> {
        // Demo mode: Find in sample data
        let jobs = Self::get_sample_jobs();
        Ok(jobs.into_iter().find(|j| j.id == JobId(job_id)))
    }
    
    /// Create a new job posting (Demo mode)
    pub async fn create_job(
        request: CreateJobRequest,
        _user_id: Uuid,
    ) -> Result<SharedJob, shared::errors::AppError> {
        // Demo mode: Create a new job with provided data
        let job = SharedJob {
            id: JobId::new(),
            title: request.title,
            description: request.description,
            pharmacy_name: request.pharmacy_name,
            hourly_rate: request.hourly_rate,
            address: request.address,
            suburb: request.suburb,
            postcode: Postcode::new(&request.postcode)?,
            state: AustralianState::from_abbrev(&request.state).ok_or_else(|| {
                shared::errors::AppError::validation("state", "Invalid Australian state")
            })?,
            latitude: request.latitude,
            longitude: request.longitude,
            start_date: request.start_date,
            end_date: request.end_date,
            start_time: request.start_time,
            end_time: request.end_time,
            job_type: request.job_type,
            status: JobStatus::Active,
            is_urgent: request.is_urgent,
            distance_km: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            created_by: UserId(_user_id),
        };
        
        Ok(job)
    }
    
    /// Update an existing job (Demo mode)
    pub async fn update_job(
        job_id: Uuid,
        request: CreateJobRequest,
    ) -> Result<Option<SharedJob>, shared::errors::AppError> {
        // Demo mode: Create updated job if it exists in sample data
        let jobs = Self::get_sample_jobs();
        if jobs.iter().any(|j| j.id == JobId(job_id)) {
            let job = SharedJob {
                id: JobId(job_id),
                title: request.title,
                description: request.description,
                pharmacy_name: request.pharmacy_name,
                hourly_rate: request.hourly_rate,
                address: request.address,
                suburb: request.suburb,
                postcode: Postcode::new(request.postcode)?,
                state: AustralianState::from_abbrev(&request.state).ok_or_else(|| {
                    shared::errors::AppError::validation("state", "Invalid Australian state")
                })?,
                latitude: request.latitude,
                longitude: request.longitude,
                start_date: request.start_date,
                end_date: request.end_date,
                start_time: request.start_time,
                end_time: request.end_time,
                job_type: request.job_type,
                status: JobStatus::Active,
                is_urgent: request.is_urgent,
                distance_km: None,
                created_at: chrono::Utc::now() - chrono::Duration::days(5),
                updated_at: chrono::Utc::now(),
                created_by: UserId(Uuid::new_v4()),
            };
            Ok(Some(job))
        } else {
            Ok(None)
        }
    }
    
    /// Delete a job (Demo mode)
    pub async fn delete_job(
        job_id: Uuid,
    ) -> Result<bool, shared::errors::AppError> {
        // Demo mode: Check if job exists in sample data
        let jobs = Self::get_sample_jobs();
        Ok(jobs.iter().any(|j| j.id == JobId(job_id)))
    }
    
    /// Get sample jobs for demo mode
    fn get_sample_jobs() -> Vec<SharedJob> {
        vec![
            SharedJob {
                id: JobId(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap()),
                title: "Senior Pharmacist - Bondi Beach".to_string(),
                description: "Looking for an experienced pharmacist to join our busy beachside pharmacy.".to_string(),
                pharmacy_name: "Bondi Health Pharmacy".to_string(),
                hourly_rate: 42.5,
                address: "123 Campbell Parade".to_string(),
                suburb: "Bondi Beach".to_string(),
                postcode: Postcode::new("2026").unwrap(),
                state: AustralianState::NewSouthWales,
                latitude: Some(-33.8908),
                longitude: Some(151.2743),
                start_date: chrono::Utc::now() + chrono::Duration::days(7),
                end_date: chrono::Utc::now() + chrono::Duration::days(30),
                start_time: "09:00".to_string(),
                end_time: "17:00".to_string(),
                job_type: JobType::Pharmacist,
                status: JobStatus::Active,
                is_urgent: false,
                distance_km: None,
                created_at: chrono::Utc::now() - chrono::Duration::days(2),
                updated_at: chrono::Utc::now() - chrono::Duration::days(2),
                created_by: UserId(Uuid::new_v4()),
            },
            SharedJob {
                id: JobId(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap()),
                title: "Pharmacy Assistant - Parramatta".to_string(),
                description: "Great opportunity for a pharmacy assistant in a modern healthcare facility.".to_string(),
                pharmacy_name: "Westfield Medical Pharmacy".to_string(),
                hourly_rate: 28.5,
                address: "159 Church St".to_string(),
                suburb: "Parramatta".to_string(),
                postcode: Postcode::new("2150").unwrap(),
                state: AustralianState::NewSouthWales,
                latitude: Some(-33.8151),
                longitude: Some(151.0011),
                start_date: chrono::Utc::now() + chrono::Duration::days(3),
                end_date: chrono::Utc::now() + chrono::Duration::days(10),
                start_time: "08:00".to_string(),
                end_time: "16:00".to_string(),
                job_type: JobType::PharmacyAssistant,
                status: JobStatus::Active,
                is_urgent: true,
                distance_km: None,
                created_at: chrono::Utc::now() - chrono::Duration::days(1),
                updated_at: chrono::Utc::now() - chrono::Duration::days(1),
                created_by: UserId(Uuid::new_v4()),
            },
        ]
    }
}