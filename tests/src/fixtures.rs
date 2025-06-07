//! Test fixtures and mock data for Loco Platform tests

use serde_json::json;
use shared::types::{Job, JobType, JobStatus, AustralianState, JobId, UserId, Postcode};
use uuid::Uuid;
use chrono::{Utc, Duration};

/// Generate sample job data for testing
pub fn create_sample_jobs() -> Vec<Job> {
    vec![
        Job {
            id: JobId::new(),
            title: "Senior Clinical Pharmacist".to_string(),
            description: "Leading clinical pharmacy role in major Sydney hospital".to_string(),
            pharmacy_name: "Sydney Metro Health".to_string(),
            hourly_rate: 65.0,
            address: "123 George Street".to_string(),
            suburb: "Sydney".to_string(),
            postcode: Postcode::new("2000").unwrap(),
            state: AustralianState::NewSouthWales,
            latitude: Some(-33.8688),
            longitude: Some(151.2093),
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(30),
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Hospital Pharmacist".to_string(),
            description: "Exciting opportunity in Melbourne's leading hospital".to_string(),
            pharmacy_name: "Melbourne General Hospital".to_string(),
            hourly_rate: 55.0,
            address: "456 Collins Street".to_string(),
            suburb: "Melbourne".to_string(),
            postcode: Postcode::new("3000").unwrap(),
            state: AustralianState::Victoria,
            latitude: Some(-37.8136),
            longitude: Some(144.9631),
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(60),
            start_time: "08:00".to_string(),
            end_time: "16:00".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Community Pharmacist".to_string(),
            description: "Part-time role perfect for work-life balance".to_string(),
            pharmacy_name: "Brisbane Community Pharmacy".to_string(),
            hourly_rate: 40.0,
            address: "789 Queen Street".to_string(),
            suburb: "Brisbane".to_string(),
            postcode: Postcode::new("4000").unwrap(),
            state: AustralianState::Queensland,
            latitude: Some(-27.4698),
            longitude: Some(153.0251),
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(90),
            start_time: "10:00".to_string(),
            end_time: "15:00".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
    ]
}

/// Mock API responses for testing
pub fn mock_jobs_api_response() -> serde_json::Value {
    let jobs = create_sample_jobs();
    json!({
        "jobs": jobs,
        "total": jobs.len(),
        "page": 1,
        "per_page": 10
    })
}

/// Mock health check response
pub fn mock_health_check_response() -> serde_json::Value {
    json!({
        "status": "ok",
        "timestamp": Utc::now().to_rfc3339(),
        "version": "1.0.0",
        "environment": "test"
    })
}

/// Mock user profile data
pub fn mock_user_profile() -> serde_json::Value {
    json!({
        "id": Uuid::new_v4(),
        "email": "test@example.com",
        "first_name": "Test",
        "last_name": "User",
        "phone": "+61412345678",
        "registration_number": "PHM123456",
        "experience_level": "Mid",
        "preferred_locations": ["Sydney", "Melbourne"],
        "availability": {
            "monday": { "start": "09:00", "end": "17:00" },
            "tuesday": { "start": "09:00", "end": "17:00" },
            "wednesday": { "start": "09:00", "end": "17:00" },
            "thursday": { "start": "09:00", "end": "17:00" },
            "friday": { "start": "09:00", "end": "17:00" }
        }
    })
}

/// Mock search filters for testing
pub fn mock_search_filters() -> serde_json::Value {
    json!({
        "location": "Sydney",
        "job_type": "FullTime",
        "experience_level": "Mid",
        "salary_min": 70000,
        "salary_max": 100000,
        "page": 1,
        "per_page": 10
    })
}

/// Mock WebSocket messages for real-time testing
pub fn mock_websocket_messages() -> Vec<serde_json::Value> {
    vec![
        json!({
            "type": "job_posted",
            "data": {
                "job_id": Uuid::new_v4(),
                "title": "Urgent: Emergency Pharmacist Needed",
                "location": "Sydney CBD",
                "urgency": "high"
            }
        }),
        json!({
            "type": "application_update",
            "data": {
                "application_id": Uuid::new_v4(),
                "status": "interview_scheduled",
                "message": "Interview scheduled for tomorrow at 2pm"
            }
        }),
        json!({
            "type": "system_notification",
            "data": {
                "message": "System maintenance scheduled for tonight",
                "severity": "info"
            }
        }),
    ]
}

/// Test user credentials
pub struct TestUser {
    pub email: String,
    pub password: String,
    pub role: String,
}

impl TestUser {
    pub fn admin() -> Self {
        Self {
            email: "admin@test.com".to_string(),
            password: "admin123".to_string(),
            role: "admin".to_string(),
        }
    }
    
    pub fn practitioner() -> Self {
        Self {
            email: "practitioner@test.com".to_string(),
            password: "practitioner123".to_string(),
            role: "practitioner".to_string(),
        }
    }
    
    pub fn employer() -> Self {
        Self {
            email: "employer@test.com".to_string(),
            password: "employer123".to_string(),
            role: "employer".to_string(),
        }
    }
}

/// Generate test form data
pub fn generate_job_form_data() -> serde_json::Value {
    json!({
        "title": "Test Pharmacist Position",
        "description": "This is a test job posting for automated testing purposes",
        "company_name": "Test Healthcare Group",
        "location": "Sydney, NSW",
        "postcode": "2000",
        "state": "NSW",
        "job_type": "FullTime",
        "experience_level": "Mid",
        "salary_min": "75000",
        "salary_max": "95000",
        "requirements": [
            "Bachelor of Pharmacy",
            "Current AHPRA registration",
            "2+ years hospital experience"
        ],
        "benefits": [
            "Competitive salary",
            "Professional development",
            "Flexible working arrangements"
        ]
    })
}

/// Generate test application data
pub fn generate_application_data() -> serde_json::Value {
    json!({
        "job_id": Uuid::new_v4(),
        "cover_letter": "I am very interested in this position and believe my skills align perfectly with your requirements.",
        "availability_start": "2024-02-01",
        "salary_expectation": 85000,
        "additional_notes": "Available for immediate start if required."
    })
}