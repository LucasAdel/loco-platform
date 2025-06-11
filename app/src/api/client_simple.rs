use leptos::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

const API_BASE_URL: &str = "/api";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub description: String,
    pub salary_min: Option<f64>,
    pub salary_max: Option<f64>,
    pub job_type: String,
    pub posted_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}

/// Fetch jobs from the API
pub async fn fetch_jobs() -> Result<Vec<Job>, String> {
    let response = Request::get(&format!("{}/v1/jobs", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch jobs: {}", e))?;
    
    if response.ok() {
        response
            .json::<Vec<Job>>()
            .await
            .map_err(|e| format!("Failed to parse jobs: {}", e))
    } else {
        Err(format!("API error: {}", response.status()))
    }
}

/// Login to the API
pub async fn login(email: String, password: String) -> Result<LoginResponse, String> {
    let login_req = LoginRequest { email, password };
    
    let response = Request::post(&format!("{}/v1/auth/login", API_BASE_URL))
        .json(&login_req)
        .map_err(|e| format!("Failed to serialize login request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to send login request: {}", e))?;
    
    if response.ok() {
        response
            .json::<LoginResponse>()
            .await
            .map_err(|e| format!("Failed to parse login response: {}", e))
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Login failed ({}): {}", status, error_text))
    }
}

/// Search jobs
pub async fn search_jobs(query: String) -> Result<Vec<Job>, String> {
    let response = Request::post(&format!("{}/v1/jobs/search", API_BASE_URL))
        .json(&serde_json::json!({ "query": query }))
        .map_err(|e| format!("Failed to serialize search request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to send search request: {}", e))?;
    
    if response.ok() {
        response
            .json::<Vec<Job>>()
            .await
            .map_err(|e| format!("Failed to parse search results: {}", e))
    } else {
        Err(format!("Search failed: {}", response.status()))
    }
}

/// Get current user profile
pub async fn get_profile(token: &str) -> Result<User, String> {
    let response = Request::get(&format!("{}/v1/profile", API_BASE_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Failed to fetch profile: {}", e))?;
    
    if response.ok() {
        response
            .json::<User>()
            .await
            .map_err(|e| format!("Failed to parse profile: {}", e))
    } else {
        Err(format!("Failed to get profile: {}", response.status()))
    }
}