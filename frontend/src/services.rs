use shared::{Job, SearchRequest, SearchResponse, CreateJobRequest, AppError};
use reqwest::Client;
use serde_json;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:3000/api".to_string(),
        }
    }
    
    pub async fn search_jobs(&self, request: SearchRequest) -> Result<SearchResponse, AppError> {
        let response = self
            .client
            .post(&format!("{}/jobs/search", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "job_api".to_string(),
                message: e.to_string(),
                status_code: None,
                retry_after: None,
            })?;
            
        if response.status().is_success() {
            response
                .json::<SearchResponse>()
                .await
                .map_err(|e| AppError::ExternalService {
                    service: "job_api".to_string(),
                    message: e.to_string(),
                    status_code: None,
                    retry_after: None,
                })
        } else {
            Err(AppError::ExternalService {
                service: "job_api".to_string(),
                message: format!("API request failed with status: {}", response.status()),
                status_code: Some(response.status().as_u16()),
                retry_after: None,
            })
        }
    }
    
    pub async fn get_jobs(&self) -> Result<Vec<Job>, AppError> {
        let response = self
            .client
            .get(&format!("{}/jobs", self.base_url))
            .send()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "job_api".to_string(),
                message: e.to_string(),
                status_code: None,
                retry_after: None,
            })?;
            
        if response.status().is_success() {
            response
                .json::<Vec<Job>>()
                .await
                .map_err(|e| AppError::ExternalService {
                    service: "job_api".to_string(),
                    message: e.to_string(),
                    status_code: None,
                    retry_after: None,
                })
        } else {
            Err(AppError::ExternalService {
                service: "job_api".to_string(),
                message: format!("API request failed with status: {}", response.status()),
                status_code: Some(response.status().as_u16()),
                retry_after: None,
            })
        }
    }
    
    pub async fn create_job(&self, job_request: CreateJobRequest) -> Result<Job, AppError> {
        let response = self
            .client
            .post(&format!("{}/jobs", self.base_url))
            .json(&job_request)
            .send()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "job_api".to_string(),
                message: e.to_string(),
                status_code: None,
                retry_after: None,
            })?;
            
        if response.status().is_success() {
            response
                .json::<Job>()
                .await
                .map_err(|e| AppError::ExternalService {
                    service: "job_api".to_string(),
                    message: e.to_string(),
                    status_code: None,
                    retry_after: None,
                })
        } else {
            Err(AppError::ExternalService {
                service: "job_api".to_string(),
                message: format!("API request failed with status: {}", response.status()),
                status_code: Some(response.status().as_u16()),
                retry_after: None,
            })
        }
    }
    
    pub async fn get_job(&self, job_id: &str) -> Result<Job, AppError> {
        let response = self
            .client
            .get(&format!("{}/jobs/{}", self.base_url, job_id))
            .send()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "job_api".to_string(),
                message: e.to_string(),
                status_code: None,
                retry_after: None,
            })?;
            
        if response.status().is_success() {
            response
                .json::<Job>()
                .await
                .map_err(|e| AppError::ExternalService {
                    service: "job_api".to_string(),
                    message: e.to_string(),
                    status_code: None,
                    retry_after: None,
                })
        } else if response.status() == 404 {
            Err(AppError::NotFound {
                resource_type: "Job".to_string(),
                id: job_id.to_string(),
                suggestions: vec!["Try refreshing the page".to_string()],
            })
        } else {
            Err(AppError::ExternalService {
                service: "job_api".to_string(),
                message: format!("API request failed with status: {}", response.status()),
                status_code: Some(response.status().as_u16()),
                retry_after: None,
            })
        }
    }
}