use reqwest::Client;
use serde_json::json;
use shared::types::{Application, ApplicationStatus, CreateApplicationRequest, UpdateApplicationRequest};
use uuid::Uuid;

const API_BASE_URL: &str = "http://localhost:3070/api/v1";

pub async fn fetch_applications(
    job_id: Option<Uuid>,
    user_id: Option<Uuid>,
) -> Result<Vec<Application>, String> {
    let client = Client::new();
    
    let mut url = format!("{}/applications", API_BASE_URL);
    let mut params = Vec::new();
    
    if let Some(job_id) = job_id {
        params.push(format!("job_id={}", job_id));
    }
    
    if let Some(user_id) = user_id {
        params.push(format!("user_id={}", user_id));
    }
    
    if !params.is_empty() {
        url = format!("{}?{}", url, params.join("&"));
    }

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        let applications: Vec<Application> = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;
        Ok(applications)
    } else {
        // Return sample data for demo mode
        Ok(generate_sample_applications(job_id, user_id))
    }
}

pub async fn create_application(request: CreateApplicationRequest) -> Result<Application, String> {
    let client = Client::new();
    
    let response = client
        .post(&format!("{}/applications", API_BASE_URL))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        let application: Application = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;
        Ok(application)
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to create application: {}", error_text))
    }
}

pub async fn update_application_status(
    application_id: Uuid,
    status: ApplicationStatus,
) -> Result<Application, String> {
    let client = Client::new();
    
    let response = client
        .put(&format!("{}/applications/{}/status", API_BASE_URL, application_id))
        .json(&json!({ "status": status }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        let application: Application = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;
        Ok(application)
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to update application status: {}", error_text))
    }
}

pub async fn get_application(application_id: Uuid) -> Result<Application, String> {
    let client = Client::new();
    
    let response = client
        .get(&format!("{}/applications/{}", API_BASE_URL, application_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        let application: Application = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;
        Ok(application)
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to get application: {}", error_text))
    }
}

pub async fn update_application(
    application_id: Uuid,
    request: UpdateApplicationRequest,
) -> Result<Application, String> {
    let client = Client::new();
    
    let response = client
        .put(&format!("{}/applications/{}", API_BASE_URL, application_id))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        let application: Application = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;
        Ok(application)
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to update application: {}", error_text))
    }
}

pub async fn delete_application(application_id: Uuid) -> Result<(), String> {
    let client = Client::new();
    
    let response = client
        .delete(&format!("{}/applications/{}", API_BASE_URL, application_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to delete application: {}", error_text))
    }
}

pub async fn withdraw_application(application_id: Uuid) -> Result<Application, String> {
    let client = Client::new();
    
    let response = client
        .put(&format!("{}/applications/{}/withdraw", API_BASE_URL, application_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        let application: Application = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;
        Ok(application)
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to withdraw application: {}", error_text))
    }
}

pub async fn get_application_stats() -> Result<serde_json::Value, String> {
    let client = Client::new();
    
    let response = client
        .get(&format!("{}/applications/stats", API_BASE_URL))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status().is_success() {
        let stats: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))?;
        Ok(stats)
    } else {
        // Return sample stats for demo mode
        Ok(json!({
            "total_applications": 156,
            "pending": 23,
            "reviewing": 18,
            "shortlisted": 12,
            "interviewed": 8,
            "offered": 4,
            "accepted": 3,
            "rejected": 85,
            "withdrawn": 3
        }))
    }
}

// Generate sample applications for demo mode
fn generate_sample_applications(job_id: Option<Uuid>, user_id: Option<Uuid>) -> Vec<Application> {
    use chrono::{Utc, Duration};

    let applications = vec![
        Application {
            id: Uuid::new_v4(),
            job_id: job_id.unwrap_or_else(Uuid::new_v4),
            user_id: user_id.unwrap_or_else(Uuid::new_v4),
            cover_letter: Some("I am excited to apply for this pharmacy position. With 5 years of experience in retail pharmacy and a passion for patient care, I believe I would be a great fit for your team.".to_string()),
            status: ApplicationStatus::Pending,
            applied_at: Utc::now() - Duration::hours(2),
            updated_at: Utc::now() - Duration::hours(2),
        },
        Application {
            id: Uuid::new_v4(),
            job_id: job_id.unwrap_or_else(Uuid::new_v4),
            user_id: user_id.unwrap_or_else(Uuid::new_v4),
            cover_letter: Some("I have been working as a locum pharmacist for the past 3 years and am looking for a more permanent position. I have excellent communication skills and am comfortable working in fast-paced environments.".to_string()),
            status: ApplicationStatus::Reviewing,
            applied_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now() - Duration::hours(6),
        },
        Application {
            id: Uuid::new_v4(),
            job_id: job_id.unwrap_or_else(Uuid::new_v4),
            user_id: user_id.unwrap_or_else(Uuid::new_v4),
            cover_letter: Some("As a recent pharmacy graduate with intern experience at two major retail chains, I am eager to continue my career in community pharmacy. I am particularly interested in this role because of your focus on clinical services.".to_string()),
            status: ApplicationStatus::Shortlisted,
            applied_at: Utc::now() - Duration::days(2),
            updated_at: Utc::now() - Duration::hours(4),
        },
        Application {
            id: Uuid::new_v4(),
            job_id: job_id.unwrap_or_else(Uuid::new_v4),
            user_id: user_id.unwrap_or_else(Uuid::new_v4),
            cover_letter: Some("I have been working in hospital pharmacy for 8 years and am now looking to transition to community pharmacy. I bring strong clinical knowledge and medication management experience.".to_string()),
            status: ApplicationStatus::Interviewed,
            applied_at: Utc::now() - Duration::days(3),
            updated_at: Utc::now() - Duration::hours(1),
        },
        Application {
            id: Uuid::new_v4(),
            job_id: job_id.unwrap_or_else(Uuid::new_v4),
            user_id: user_id.unwrap_or_else(Uuid::new_v4),
            cover_letter: Some("I am a pharmacy assistant with 4 years of experience and am currently studying to become a qualified pharmacist. I am very detail-oriented and have excellent customer service skills.".to_string()),
            status: ApplicationStatus::Offered,
            applied_at: Utc::now() - Duration::days(5),
            updated_at: Utc::now() - Duration::minutes(30),
        },
        Application {
            id: Uuid::new_v4(),
            job_id: job_id.unwrap_or_else(Uuid::new_v4),
            user_id: user_id.unwrap_or_else(Uuid::new_v4),
            cover_letter: Some("Thank you for offering me this position. I am delighted to accept and look forward to starting with your pharmacy team next Monday.".to_string()),
            status: ApplicationStatus::Accepted,
            applied_at: Utc::now() - Duration::days(7),
            updated_at: Utc::now() - Duration::minutes(10),
        },
        Application {
            id: Uuid::new_v4(),
            job_id: job_id.unwrap_or_else(Uuid::new_v4),
            user_id: user_id.unwrap_or_else(Uuid::new_v4),
            cover_letter: Some("I applied for this position but have since accepted another role. Thank you for your time and consideration.".to_string()),
            status: ApplicationStatus::Withdrawn,
            applied_at: Utc::now() - Duration::days(4),
            updated_at: Utc::now() - Duration::hours(12),
        },
    ];

    applications
}