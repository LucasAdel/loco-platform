use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    entities::{
        application::{self, ApplicationStatus},
        job,
        user,
    },
    repository::PaginationParams,
    services::application_service::ApplicationService,
    error::AppError,
    middleware::auth::AuthContext,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateApplicationRequest {
    pub job_id: Uuid,
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
    pub availability_note: Option<String>,
    pub experience_years: Option<i32>,
    pub registration_number: Option<String>,
    pub preferred_contact_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApplicationRequest {
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
    pub availability_note: Option<String>,
    pub experience_years: Option<i32>,
    pub registration_number: Option<String>,
    pub preferred_contact_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApplicationStatusRequest {
    pub status: ApplicationStatus,
    pub reviewer_notes: Option<String>,
    pub interview_scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationFilters {
    pub job_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub status: Option<ApplicationStatus>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    #[serde(default)]
    pub include_withdrawn: bool,
}

#[derive(Debug, Serialize)]
pub struct ApplicationResponse {
    pub id: Uuid,
    pub job_id: Uuid,
    pub user_id: Uuid,
    pub cover_letter: Option<String>,
    pub resume_url: Option<String>,
    pub availability_note: Option<String>,
    pub experience_years: Option<i32>,
    pub registration_number: Option<String>,
    pub preferred_contact_method: Option<String>,
    pub status: ApplicationStatus,
    pub status_display: String,
    pub reviewer_notes: Option<String>,
    pub interview_scheduled_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<Uuid>,
    pub applied_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub age_in_days: i64,
    pub can_be_withdrawn: bool,
    pub is_pending: bool,
    pub is_reviewed: bool,
    pub is_successful: bool,
    pub is_closed: bool,
    pub has_interview_scheduled: bool,
    pub possible_next_statuses: Vec<ApplicationStatus>,
}

#[derive(Debug, Serialize)]
pub struct ApplicationWithJobAndUser {
    #[serde(flatten)]
    pub application: ApplicationResponse,
    pub job: Option<JobSummary>,
    pub user: Option<UserSummary>,
}

#[derive(Debug, Serialize)]
pub struct JobSummary {
    pub id: Uuid,
    pub title: String,
    pub pharmacy_name: String,
    pub suburb: String,
    pub state: String,
    pub postcode: String,
    pub hourly_rate: rust_decimal::Decimal,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_urgent: bool,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub user_type: String,
}

#[derive(Debug, Serialize)]
pub struct ApplicationStatsResponse {
    pub total_applications: i64,
    pub pending_count: i64,
    pub reviewing_count: i64,
    pub shortlisted_count: i64,
    pub interviewed_count: i64,
    pub offered_count: i64,
    pub accepted_count: i64,
    pub rejected_count: i64,
    pub withdrawn_count: i64,
    pub avg_review_time_hours: Option<f64>,
    pub applications_today: i64,
    pub applications_this_week: i64,
    pub conversion_rate: f64, // accepted / total
}

/// Create a new job application
pub async fn create_application(
    State(state): State<crate::AppState>,
    auth: AuthContext,
    Json(req): Json<CreateApplicationRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Convert handler request to service request 
    let service_req = shared::types::CreateApplicationRequest {
        job_id: req.job_id,
        cover_letter: req.cover_letter,
    };
    
    let application = state.application_service
        .create_application(service_req, auth.user_id())
        .await?;
    
    let response = ApplicationResponse::from_model(application);
    Ok((StatusCode::CREATED, Json(response)))
}

/// Get application by ID with optional job and user details
pub async fn get_application(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(application_id): Path<Uuid>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let include_job = params.get("include_job").map(|v| v == "true").unwrap_or(false);
    let include_user = params.get("include_user").map(|v| v == "true").unwrap_or(false);

    let application = state.application_service
        .get_application_by_id(application_id, Some(auth.user_id()))
        .await?;

    // For now, just return the basic application
    // TODO: Implement get_application_with_details for including job and user data
    let response = ApplicationResponse::from_model(application);
    Ok(Json(ApplicationWithJobAndUser {
        application: response,
        job: None,
        user: None,
    }))
}

/// List applications with filtering and pagination
#[axum::debug_handler]
pub async fn list_applications(
    Query(filters): Query<ApplicationFilters>,
    Query(pagination): Query<PaginationParams>,
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let applications = state.application_service
        .list_applications(filters, pagination, auth.user_id())
        .await?;

    let response: Vec<ApplicationResponse> = applications
        .into_iter()
        .map(ApplicationResponse::from_model)
        .collect();

    Ok(Json(response))
}

/// Update application details (for applicants)
pub async fn update_application(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(application_id): Path<Uuid>,
    Json(req): Json<UpdateApplicationRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Convert handler request to service request
    let service_req = shared::types::UpdateApplicationRequest {
        status: None, // Only allow status updates through the dedicated endpoint
        cover_letter: req.cover_letter,
    };
    
    let application = state.application_service
        .update_application(application_id, service_req, Some(auth.user_id()))
        .await?;

    let response = ApplicationResponse::from_model(application);
    Ok(Json(response))
}

/// Update application status (for employers/admins)
pub async fn update_application_status(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(application_id): Path<Uuid>,
    Json(req): Json<UpdateApplicationStatusRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Convert to shared ApplicationStatus type
    let shared_status = match req.status {
        ApplicationStatus::Pending => shared::types::ApplicationStatus::Pending,
        ApplicationStatus::Reviewing => shared::types::ApplicationStatus::Reviewing,
        ApplicationStatus::Shortlisted => shared::types::ApplicationStatus::Shortlisted,
        ApplicationStatus::Interviewed => shared::types::ApplicationStatus::Interviewed,
        ApplicationStatus::Offered => shared::types::ApplicationStatus::Offered,
        ApplicationStatus::Accepted => shared::types::ApplicationStatus::Accepted,
        ApplicationStatus::Rejected => shared::types::ApplicationStatus::Rejected,
        ApplicationStatus::Withdrawn => shared::types::ApplicationStatus::Withdrawn,
    };
    
    let application = state.application_service
        .update_application_status(application_id, shared_status, Some(auth.user_id()))
        .await?;

    let response = ApplicationResponse::from_model(application);
    Ok(Json(response))
}

/// Withdraw application (for applicants)
pub async fn withdraw_application(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(application_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let application = state.application_service
        .withdraw_application(application_id, auth.user_id())
        .await?;

    let response = ApplicationResponse::from_model(application);
    Ok(Json(response))
}

/// Delete application (admin only)
pub async fn delete_application(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(application_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.application_service
        .delete_application(application_id, Some(auth.user_id()))
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Get applications for a specific job (employers/admins)
#[axum::debug_handler]
pub async fn get_job_applications(
    Path(job_id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let applications = state.application_service
        .get_applications_for_job(job_id, pagination, auth.user_id())
        .await?;

    // TODO: Fix ApplicationWithJobAndUser mapping
    let response: Vec<ApplicationWithJobAndUser> = applications
        .into_iter()
        .map(|app| ApplicationWithJobAndUser {
            application: ApplicationResponse::from_model(app),
            job: None,
            user: None,
        })
        .collect();

    Ok(Json(response))
}

/// Get applications by a specific user
#[axum::debug_handler]
pub async fn get_user_applications(
    Path(user_id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    // Users can only see their own applications unless they're admin
    if user_id != auth.user_id() && !auth.is_admin() {
        return Err(AppError::Forbidden);
    }

    let applications = state.application_service
        .get_applications_by_user(user_id, pagination)
        .await?;

    // TODO: Fix ApplicationWithJobAndUser mapping
    let response: Vec<ApplicationWithJobAndUser> = applications
        .into_iter()
        .map(|app| ApplicationWithJobAndUser {
            application: ApplicationResponse::from_model(app),
            job: None,
            user: None,
        })
        .collect();

    Ok(Json(response))
}

/// Get application statistics (employers/admins)
pub async fn get_application_stats(
    Query(filters): Query<ApplicationFilters>,
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let stats = state.application_service
        .get_application_statistics(Some(auth.user_id()))
        .await?;

    Ok(Json(stats))
}

impl ApplicationResponse {
    pub fn from_model(model: application::Model) -> Self {
        let status_display = model.status_display().to_string();
        let age_in_days = model.age_in_days();
        let can_be_withdrawn = model.can_be_withdrawn();
        let is_pending = model.is_pending();
        let is_reviewed = model.is_reviewed();
        let is_successful = model.is_successful();
        let is_closed = model.is_closed();
        let has_interview_scheduled = model.has_interview_scheduled();
        let possible_next_statuses = model.possible_next_statuses();
        
        Self {
            id: model.id,
            job_id: model.job_id,
            user_id: model.user_id,
            cover_letter: model.cover_letter,
            resume_url: model.resume_url,
            availability_note: model.availability_note,
            experience_years: model.experience_years,
            registration_number: model.registration_number,
            preferred_contact_method: model.preferred_contact_method,
            status: model.status,
            status_display,
            reviewer_notes: model.reviewer_notes,
            interview_scheduled_at: model.interview_scheduled_at.map(|dt| dt.into()),
            reviewed_at: model.reviewed_at.map(|dt| dt.into()),
            reviewed_by: model.reviewed_by,
            applied_at: model.applied_at.into(),
            updated_at: model.updated_at.into(),
            age_in_days,
            can_be_withdrawn,
            is_pending,
            is_reviewed,
            is_successful,
            is_closed,
            has_interview_scheduled,
            possible_next_statuses,
        }
    }
}

impl ApplicationWithJobAndUser {
    pub fn from_tuple(
        (application, job, user): (application::Model, Option<job::Model>, Option<user::Model>),
    ) -> Self {
        Self {
            application: ApplicationResponse::from_model(application),
            job: job.map(JobSummary::from_model),
            user: user.map(UserSummary::from_model),
        }
    }
}

impl JobSummary {
    pub fn from_model(model: job::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            pharmacy_name: model.pharmacy_name,
            suburb: model.suburb,
            state: model.state,
            postcode: model.postcode,
            hourly_rate: model.hourly_rate,
            start_date: model.start_date.into(),
            end_date: model.end_date.into(),
            is_urgent: model.is_urgent,
            status: format!("{:?}", model.status),
        }
    }
}

impl UserSummary {
    pub fn from_model(model: user::Model) -> Self {
        Self {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            email: model.email,
            phone: model.phone,
            user_type: format!("{:?}", model.user_type),
        }
    }
}