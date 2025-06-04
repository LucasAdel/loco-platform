use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use uuid::Uuid;

use shared::types::{SearchRequest, SearchResponse, CreateJobRequest, JobFilters};
use crate::{AppState, AppError};

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

/// Create a new job posting
pub async fn create_job(
    State(_state): State<AppState>,
    Json(request): Json<CreateJobRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Demo mode: Use service
    // TODO: Add authentication to get user_id
    let user_id = Uuid::new_v4(); // Mock user ID for demo
    let job = crate::services::JobService::create_job(request, user_id).await?;
    
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