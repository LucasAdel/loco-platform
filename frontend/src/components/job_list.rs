use dioxus::prelude::*;
use shared::types::{Job, JobType, JobId, UserId, Postcode, AustralianState, JobStatus};
use crate::components::job_card::JobCard;
use chrono::Utc;

#[derive(Props, Clone, PartialEq)]
pub struct JobListProps {
    jobs: Vec<Job>,
}

#[component]
pub fn JobList(props: JobListProps) -> Element {
    let jobs_to_display = if props.jobs.is_empty() {
        get_sample_jobs()
    } else {
        props.jobs
    };

    rsx! {
        div { class: "job-list space-y-4",
            h2 { class: "text-2xl font-bold mb-6 text-gray-800",
                "Available Positions"
            }
            
            if jobs_to_display.is_empty() {
                div { class: "text-center py-12 bg-gray-50 rounded-lg",
                    div { class: "text-gray-500 text-lg mb-2",
                        "🔍 No jobs found"
                    }
                    p { class: "text-gray-400",
                        "Try adjusting your search criteria"
                    }
                }
            } else {
                div { class: "grid gap-4",
                    for job in jobs_to_display {
                        JobCard { job: job }
                    }
                }
            }
        }
    }
}

pub fn get_mock_jobs() -> Vec<Job> {
    get_sample_jobs()
}

fn get_sample_jobs() -> Vec<Job> {
    vec![
        Job {
            id: JobId::new(),
            title: "Intern Position - Adelaide".to_string(),
            description: "Great opportunity for pharmacy intern in busy city location".to_string(),
            pharmacy_name: "City Pharmacy".to_string(),
            hourly_rate: 39.0,
            address: "1 King William Street, Adelaide".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode::new("5000").unwrap(),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9285),
            longitude: Some(138.6007),
            start_date: Utc::now(),
            end_date: Utc::now() + chrono::Duration::days(30),
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            job_type: JobType::Intern,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(2.5),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Senior Pharmacist - Melbourne".to_string(),
            description: "Looking for experienced pharmacist to lead our team".to_string(),
            pharmacy_name: "Wellness Pharmacy".to_string(),
            hourly_rate: 55.0,
            address: "123 Collins Street, Melbourne".to_string(),
            suburb: "Melbourne".to_string(),
            postcode: Postcode::new("3000").unwrap(),
            state: AustralianState::Victoria,
            latitude: Some(-37.8136),
            longitude: Some(144.9631),
            start_date: Utc::now(),
            end_date: Utc::now() + chrono::Duration::days(60),
            start_time: "08:30".to_string(),
            end_time: "18:00".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: Some(8.7),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
    ]
}