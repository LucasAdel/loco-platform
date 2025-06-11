use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Json, IntoResponse},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    entities::job,
    services::EnhancedJobService,
    middleware::auth::AuthContext,
    repository::PaginationParams,
    AppError, AppState,
};
use shared::types::{CreateJobRequest, JobFilters, AustralianState};
use rust_decimal::Decimal;
use num_traits::ToPrimitive;

/// Create a new job posting
pub async fn create_job(
    State(state): State<AppState>,
    auth: AuthContext,
    Json(req): Json<CreateJobRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    let job = service.create_job(req, auth.user_id()).await?;
    
    let response = JobResponse::from_model(job);
    Ok((StatusCode::CREATED, Json(response)))
}

/// Get job by ID
pub async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    let job = service.get_job_by_id(job_id).await?;
    
    let response = JobResponse::from_model(job);
    Ok(Json(response))
}

/// List jobs with filtering and pagination
pub async fn list_jobs(
    State(state): State<AppState>,
    Query(filters): Query<JobFilters>,
    Query(pagination): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    let (jobs, total) = service.list_jobs(filters, pagination.clone()).await?;
    
    let job_responses: Vec<JobResponse> = jobs
        .into_iter()
        .map(JobResponse::from_model)
        .collect();
    
    let response = JobListResponse {
        jobs: job_responses,
        total,
        page: pagination.page(),
        page_size: pagination.page_size(),
    };
    
    Ok(Json(response))
}

/// Update job status
pub async fn update_job_status(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(job_id): Path<Uuid>,
    Json(req): Json<UpdateJobStatusRequest>,
) -> Result<impl IntoResponse, AppError> {
    // For now, return a simple success response since the enhanced service needs work
    Ok(Json(serde_json::json!({
        "message": "Job status update functionality coming soon",
        "job_id": job_id,
        "status": format!("{:?}", req.status)
    })))
}

/// Delete job
pub async fn delete_job(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(job_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    service.delete_job(job_id, auth.user_id()).await?;
    
    Ok(StatusCode::NO_CONTENT)
}

/// Get jobs posted by the authenticated user
pub async fn get_my_jobs(
    State(state): State<AppState>,
    auth: AuthContext,
    Query(pagination): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    let (jobs, total) = service.get_jobs_by_user(auth.user_id(), pagination.clone()).await?;
    
    let job_responses: Vec<JobResponse> = jobs
        .into_iter()
        .map(JobResponse::from_model)
        .collect();
    
    let response = JobListResponse {
        jobs: job_responses,
        total,
        page: pagination.page(),
        page_size: pagination.page_size(),
    };
    
    Ok(Json(response))
}

/// Search jobs by text
pub async fn search_jobs(
    State(state): State<AppState>,
    Query(params): Query<SearchJobsQuery>,
    Query(filters): Query<JobFilters>,
    Query(pagination): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    let query = params.q.unwrap_or_default();
    let (jobs, total) = service.search_jobs(&query, filters, pagination.clone()).await?;
    
    let job_responses: Vec<JobResponse> = jobs
        .into_iter()
        .map(JobResponse::from_model)
        .collect();
    
    let response = JobListResponse {
        jobs: job_responses,
        total,
        page: pagination.page(),
        page_size: pagination.page_size(),
    };
    
    Ok(Json(response))
}

/// Find jobs near a location
pub async fn find_jobs_nearby(
    State(state): State<AppState>,
    Query(params): Query<LocationSearchQuery>,
    Query(filters): Query<JobFilters>,
    Query(pagination): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    let radius = params.radius.unwrap_or(10.0); // Default 10km radius
    
    let (jobs, total) = service.find_jobs_near_location(
        params.lat,
        params.lng,
        radius,
        filters,
        pagination.clone(),
    ).await?;
    
    let job_responses: Vec<JobResponse> = jobs
        .into_iter()
        .map(JobResponse::from_model)
        .collect();
    
    let response = JobListResponse {
        jobs: job_responses,
        total,
        page: pagination.page(),
        page_size: pagination.page_size(),
    };
    
    Ok(Json(response))
}

/// Get job statistics
pub async fn get_job_statistics(
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    let stats = service.get_job_statistics(Some(auth.user_id())).await?;
    
    Ok(Json(stats))
}

/// Get global job statistics (admin only)
pub async fn get_global_job_statistics(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let service = EnhancedJobService::from(&state);
    let stats = service.get_job_statistics(None).await?;
    
    Ok(Json(stats))
}

// Request/Response Types

#[derive(Debug, Deserialize)]
pub struct UpdateJobStatusRequest {
    pub status: String, // Simplified for now
}

#[derive(Debug, Deserialize)]
pub struct SearchJobsQuery {
    pub q: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LocationSearchQuery {
    pub lat: f64,
    pub lng: f64,
    pub radius: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub pharmacy_name: String,
    pub hourly_rate: f64,
    pub address: String,
    pub suburb: String,
    pub postcode: String,
    pub state: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub start_time: String,
    pub end_time: String,
    pub job_type: String,
    pub status: String,
    pub is_urgent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct JobListResponse {
    pub jobs: Vec<JobResponse>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

impl JobResponse {
    pub fn from_model(job: job::Model) -> Self {
        Self {
            id: job.id,
            user_id: job.created_by, // Use created_by as user_id
            title: job.title,
            description: job.description,
            pharmacy_name: job.pharmacy_name,
            hourly_rate: job.hourly_rate.to_f64().unwrap_or(0.0), // Convert Decimal to f64
            address: job.address,
            suburb: job.suburb,
            postcode: job.postcode,
            state: job.state,
            latitude: job.latitude,
            longitude: job.longitude,
            start_date: job.start_date.naive_utc().and_utc(), // Convert DateTime<FixedOffset> to DateTime<Utc>
            end_date: job.end_date.naive_utc().and_utc(),
            start_time: job.start_time,
            end_time: job.end_time,
            job_type: format!("{:?}", job.job_type),
            status: format!("{:?}", job.status),
            is_urgent: job.is_urgent,
            created_at: job.created_at.naive_utc().and_utc(),
            updated_at: job.updated_at.naive_utc().and_utc(),
        }
    }
}