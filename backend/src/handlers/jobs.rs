use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use uuid::Uuid;
use chrono::Utc;

use shared::types::{SearchRequest, SearchResponse, CreateJobRequest, JobFilters};
use shared::validation::{ValidatedCreateJobRequest, ValidatedJobSearchRequest};
use crate::{
    AppState, 
    AppError,
    middleware::validation::ValidatedJson,
};

/// Get all jobs with optional filtering
pub async fn list_jobs(
    State(_state): State<AppState>,
    Query(filters): Query<JobFilters>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service with sample data
    let (jobs, total) = crate::services::JobService::list_jobs(filters, Some(1), Some(20)).await?;
    
    Ok(Json(SearchResponse {
        jobs,
        total_count: total,
        page: 1,
        limit: 20,
        has_more: false,
    }))
}

/// Get a specific job by ID
pub async fn get_job(
    State(_state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service
    match crate::services::JobService::get_job(job_id).await? {
        Some(job) => Ok(Json(job)),
        None => Err(AppError::NotFound),
    }
}

/// Create a new job posting with validation
pub async fn create_job(
    State(_state): State<AppState>,
    ValidatedJson(request): ValidatedJson<ValidatedCreateJobRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Convert validated request to CreateJobRequest
    let job_request = CreateJobRequest {
        title: request.title,
        description: request.description,
        pharmacy_name: request.company, // Map company to pharmacy_name
        hourly_rate: request.salary_min.map(|s| s as f64 / 40.0 / 52.0).unwrap_or(25.0), // Convert annual to hourly
        address: request.address,
        suburb: request.suburb,
        postcode: request.postcode,
        state: request.state,
        latitude: request.latitude,
        longitude: request.longitude,
        start_date: request.start_date.map(|d| d.and_hms_opt(9, 0, 0).unwrap().and_utc()).unwrap_or_else(|| Utc::now()),
        end_date: request.end_date.map(|d| d.and_hms_opt(17, 0, 0).unwrap().and_utc()).unwrap_or_else(|| Utc::now()),
        start_time: "09:00".to_string(), // Default work hours
        end_time: "17:00".to_string(),
        job_type: request.job_type,
        is_urgent: request.is_urgent,
    };
    
    // TODO: Add authentication to get user_id
    let user_id = Uuid::new_v4(); // Mock user ID for demo
    let job = crate::services::JobService::create_job(job_request, user_id).await?;
    
    Ok((StatusCode::CREATED, Json(job)))
}

/// Update an existing job
pub async fn update_job(
    State(_state): State<AppState>,
    Path(job_id): Path<Uuid>,
    Json(request): Json<CreateJobRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service
    match crate::services::JobService::update_job(job_id, request).await? {
        Some(job) => Ok(Json(job)),
        None => Err(AppError::NotFound),
    }
}

/// Delete a job
pub async fn delete_job(
    State(_state): State<AppState>,
    Path(job_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service
    if crate::services::JobService::delete_job(job_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

/// Search jobs with advanced criteria
pub async fn search_jobs(
    State(_state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service
    let (jobs, total) = crate::services::JobService::list_jobs(
        request.filters,
        request.page,
        request.limit,
    ).await?;
    
    Ok(Json(SearchResponse {
        jobs,
        total_count: total,
        page: request.page.unwrap_or(1),
        limit: request.limit.unwrap_or(20),
        has_more: false,
    }))
}