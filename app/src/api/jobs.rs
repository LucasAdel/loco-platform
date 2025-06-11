use shared::types::{SimpleJob, SimpleJobType, CreateJobRequest, Job};
use super::client::use_api_client;

pub async fn fetch_jobs() -> Result<Vec<SimpleJob>, String> {
    let client = use_api_client();
    
    match client.get::<Vec<SimpleJob>>("/api/jobs").await {
        Ok(jobs) => Ok(jobs),
        Err(_) => {
            // Fallback to mock data for development
            Ok(get_mock_jobs())
        }
    }
}

pub async fn fetch_job_by_id(id: &str) -> Result<SimpleJob, String> {
    let client = use_api_client();
    
    match client.get::<SimpleJob>(&format!("/api/jobs/{}", id)).await {
        Ok(job) => Ok(job),
        Err(_) => {
            // Fallback to mock data
            get_mock_jobs()
                .into_iter()
                .find(|j| j.id == id)
                .ok_or_else(|| "Job not found".to_string())
        }
    }
}

fn get_mock_jobs() -> Vec<SimpleJob> {
    vec![
        SimpleJob {
            id: "1".to_string(),
            title: "Senior Pharmacist".to_string(),
            company: "Melbourne Health".to_string(),
            location: "Melbourne, VIC".to_string(),
            description: "Lead pharmacist role in a major hospital setting with opportunities for clinical research.".to_string(),
            salary_range: "$90k - $110k".to_string(),
            job_type: SimpleJobType::FullTime,
            posted_date: "2 days ago".to_string(),
            urgent: true,
            latitude: Some(-37.8136),
            longitude: Some(144.9631),
        },
        SimpleJob {
            id: "2".to_string(),
            title: "Community Pharmacist".to_string(),
            company: "Terry White Chemmart".to_string(),
            location: "Sydney, NSW".to_string(),
            description: "Join our friendly team in a busy community pharmacy focused on patient care.".to_string(),
            salary_range: "$75k - $85k".to_string(),
            job_type: SimpleJobType::FullTime,
            posted_date: "1 week ago".to_string(),
            urgent: false,
            latitude: Some(-33.8688),
            longitude: Some(151.2093),
        },
        SimpleJob {
            id: "3".to_string(),
            title: "Locum Pharmacist".to_string(),
            company: "Various Locations".to_string(),
            location: "Brisbane, QLD".to_string(),
            description: "Flexible locum opportunities across Brisbane metro area.".to_string(),
            salary_range: "$50 - $60/hr".to_string(),
            job_type: SimpleJobType::Contract,
            posted_date: "3 days ago".to_string(),
            urgent: true,
            latitude: Some(-27.4698),
            longitude: Some(153.0251),
        },
    ]
}

pub async fn create_job(request: CreateJobRequest) -> Result<Job, String> {
    let client = use_api_client();
    
    match client.post::<Job, CreateJobRequest>("/api/jobs", &request).await {
        Ok(job) => Ok(job),
        Err(e) => Err(format!("Failed to create job: {:?}", e))
    }
}