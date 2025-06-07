use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
    AppState, 
    AppError,
    services::{SearchService, search_service::AdvancedSearchFilters},
    middleware::validation::ValidatedJson,
};
use shared::validation::ValidatedJobSearchRequest;

/// Advanced search endpoint with comprehensive filtering and analytics
pub async fn advanced_search(
    State(_state): State<AppState>,
    ValidatedJson(request): ValidatedJson<ValidatedJobSearchRequest>,
) -> Result<impl IntoResponse, AppError> {
    let search_service = SearchService::new();
    
    // Convert validated request to advanced filters
    let filters = AdvancedSearchFilters {
        query: request.query.clone(),
        title_keywords: None,
        company_keywords: None,
        description_keywords: None,
        job_types: request.job_types.map(|types| 
            types.into_iter().map(|t| format!("{:?}", t)).collect()
        ),
        salary_min: request.min_salary,
        salary_max: request.max_salary,
        employment_types: None,
        states: None,
        cities: None,
        postcodes: None,
        latitude: request.latitude,
        longitude: request.longitude,
        radius_km: request.radius_km,
        posted_after: None,
        posted_before: None,
        start_date_after: None,
        start_date_before: None,
        requires_ahpra: None,
        requires_vaccination: None,
        experience_level: None,
        pharmacy_type: None,
        specializations: None,
        is_urgent: request.is_urgent,
        remote_possible: request.remote_possible,
        featured_only: None,
        sort_by: Some("relevance".to_string()),
        sort_order: Some("desc".to_string()),
        page: request.page,
        limit: request.limit,
    };
    
    // Perform search
    let (scored_jobs, analytics) = search_service.search_jobs(filters).await?;
    
    // Save search for analytics (if user is authenticated)
    if let Some(query) = &request.query {
        // TODO: Extract user_id from auth context
        search_service.save_search(
            None, // user_id
            query,
            &AdvancedSearchFilters {
                query: request.query.clone(),
                ..Default::default()
            },
            analytics.total_results,
        ).await?;
    }
    
    let response = json!({
        "jobs": scored_jobs,
        "analytics": analytics,
        "pagination": {
            "page": request.page.unwrap_or(1),
            "limit": request.limit.unwrap_or(20),
            "total": analytics.total_results,
            "has_more": analytics.total_results > (request.page.unwrap_or(1) * request.limit.unwrap_or(20)) as i64
        }
    });
    
    Ok(Json(response))
}

/// Quick search endpoint for search bar
pub async fn quick_search(
    State(_state): State<AppState>,
    Query(params): Query<QuickSearchParams>,
) -> Result<impl IntoResponse, AppError> {
    let search_service = SearchService::new();
    
    let filters = AdvancedSearchFilters {
        query: Some(params.q),
        limit: Some(params.limit.unwrap_or(10)),
        page: Some(1),
        ..Default::default()
    };
    
    let (scored_jobs, _) = search_service.search_jobs(filters).await?;
    
    // Return simplified job results
    let simple_jobs: Vec<SimpleJobResult> = scored_jobs.into_iter()
        .map(|scored| SimpleJobResult {
            id: scored.job.id.0,
            title: scored.job.title,
            company: scored.job.pharmacy_name,
            location: format!("{}, {} {}", scored.job.suburb, scored.job.state, scored.job.postcode),
            salary_display: format!("${:.0}/hour", scored.job.hourly_rate),
            is_urgent: scored.job.is_urgent,
            relevance_score: scored.relevance_score,
        })
        .collect();
    
    Ok(Json(json!({
        "jobs": simple_jobs,
        "total": simple_jobs.len()
    })))
}

/// Search suggestions endpoint for autocomplete
pub async fn search_suggestions(
    State(_state): State<AppState>,
    Query(params): Query<SuggestionParams>,
) -> Result<impl IntoResponse, AppError> {
    let search_service = SearchService::new();
    
    let suggestions = search_service.get_search_suggestions(&params.q).await?;
    
    Ok(Json(json!({
        "suggestions": suggestions
    })))
}

/// Trending searches endpoint
pub async fn trending_searches(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let search_service = SearchService::new();
    
    let trending = search_service.get_trending_searches().await?;
    
    Ok(Json(json!({
        "trending": trending
    })))
}

/// Saved searches for authenticated users
pub async fn get_saved_searches(
    State(_state): State<AppState>,
    // TODO: Add auth context extraction
) -> Result<impl IntoResponse, AppError> {
    // TODO: Implement saved searches retrieval
    // For now, return empty list
    Ok(Json(json!({
        "saved_searches": []
    })))
}

/// Save a search for later
pub async fn save_search(
    State(_state): State<AppState>,
    Json(request): Json<SaveSearchRequest>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Implement search saving
    // For now, just return success
    Ok(Json(json!({
        "message": "Search saved successfully",
        "search_id": Uuid::new_v4()
    })))
}

/// Job recommendations based on user profile and search history
pub async fn job_recommendations(
    State(_state): State<AppState>,
    Query(params): Query<RecommendationParams>,
) -> Result<impl IntoResponse, AppError> {
    let search_service = SearchService::new();
    
    // In production, this would analyze user profile and preferences
    let filters = AdvancedSearchFilters {
        query: None,
        limit: Some(params.limit.unwrap_or(10)),
        page: Some(1),
        sort_by: Some("relevance".to_string()),
        ..Default::default()
    };
    
    let (scored_jobs, _) = search_service.search_jobs(filters).await?;
    
    let recommendations: Vec<JobRecommendation> = scored_jobs.into_iter()
        .map(|scored| JobRecommendation {
            job: SimpleJobResult {
                id: scored.job.id.0,
                title: scored.job.title,
                company: scored.job.pharmacy_name,
                location: format!("{}, {} {}", scored.job.suburb, scored.job.state, scored.job.postcode),
                salary_display: format!("${:.0}/hour", scored.job.hourly_rate),
                is_urgent: scored.job.is_urgent,
                relevance_score: scored.relevance_score,
            },
            match_reasons: scored.match_reasons,
            recommendation_score: scored.relevance_score,
        })
        .collect();
    
    Ok(Json(json!({
        "recommendations": recommendations,
        "total": recommendations.len()
    })))
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct QuickSearchParams {
    pub q: String,
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SuggestionParams {
    pub q: String,
}

#[derive(Debug, Deserialize)]
pub struct RecommendationParams {
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveSearchRequest {
    pub name: String,
    pub query: String,
    pub filters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimpleJobResult {
    pub id: Uuid,
    pub title: String,
    pub company: String,
    pub location: String,
    pub salary_display: String,
    pub is_urgent: bool,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobRecommendation {
    pub job: SimpleJobResult,
    pub match_reasons: Vec<String>,
    pub recommendation_score: f64,
}

// Helper functions
fn format_salary_range(min: Option<i32>, max: Option<i32>) -> String {
    match (min, max) {
        (Some(min_sal), Some(max_sal)) => {
            if min_sal == max_sal {
                format!("${}", min_sal)
            } else {
                format!("${} - ${}", min_sal, max_sal)
            }
        }
        (Some(min_sal), None) => format!("From ${}", min_sal),
        (None, Some(max_sal)) => format!("Up to ${}", max_sal),
        (None, None) => "Competitive".to_string(),
    }
}

impl Default for AdvancedSearchFilters {
    fn default() -> Self {
        Self {
            query: None,
            title_keywords: None,
            company_keywords: None,
            description_keywords: None,
            job_types: None,
            salary_min: None,
            salary_max: None,
            employment_types: None,
            states: None,
            cities: None,
            postcodes: None,
            latitude: None,
            longitude: None,
            radius_km: None,
            posted_after: None,
            posted_before: None,
            start_date_after: None,
            start_date_before: None,
            requires_ahpra: None,
            requires_vaccination: None,
            experience_level: None,
            pharmacy_type: None,
            specializations: None,
            is_urgent: None,
            remote_possible: None,
            featured_only: None,
            sort_by: None,
            sort_order: None,
            page: None,
            limit: None,
        }
    }
}