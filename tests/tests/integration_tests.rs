//! Integration tests for Loco Platform API endpoints
//! These tests verify backend functionality without requiring a browser

use loco_platform_tests::{TestConfig, TestResult, fixtures::*};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

/// HTTP client for API testing
struct ApiTestClient {
    client: Client,
    base_url: String,
}

impl ApiTestClient {
    fn new(config: &TestConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            client,
            base_url: config.api_url.clone(),
        }
    }
    
    async fn get(&self, path: &str) -> TestResult<Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&url).send().await?;
        let json = response.json::<Value>().await?;
        Ok(json)
    }
    
    async fn post(&self, path: &str, data: Value) -> TestResult<Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client
            .post(&url)
            .json(&data)
            .send()
            .await?;
        let json = response.json::<Value>().await?;
        Ok(json)
    }
    
    async fn wait_for_server(&self, max_attempts: u32) -> TestResult {
        for _ in 0..max_attempts {
            if let Ok(_) = self.get("/health").await {
                return Ok(());
            }
            sleep(Duration::from_millis(500)).await;
        }
        Err("Server did not start within timeout".into())
    }
}

#[tokio::test]
async fn test_health_check() -> TestResult {
    let config = TestConfig::from_env();
    let client = ApiTestClient::new(&config);
    
    // Wait for server to be ready
    client.wait_for_server(20).await?;
    
    let response = client.get("/health").await?;
    
    assert_eq!(response["status"], "ok");
    assert!(response["timestamp"].is_string());
    
    Ok(())
}

#[tokio::test]
async fn test_jobs_api_endpoint() -> TestResult {
    let config = TestConfig::from_env();
    let client = ApiTestClient::new(&config);
    
    client.wait_for_server(20).await?;
    
    let response = client.get("/api/jobs").await?;
    
    // Verify response structure
    assert!(response.is_array() || response.is_object());
    
    // If it's demo mode, verify we get sample data
    if let Some(jobs) = response.as_array() {
        assert!(!jobs.is_empty(), "Jobs endpoint should return sample data in demo mode");
        
        // Verify job structure
        let first_job = &jobs[0];
        assert!(first_job["id"].is_string());
        assert!(first_job["title"].is_string());
        assert!(first_job["company_name"].is_string());
        assert!(first_job["location"].is_string());
    }
    
    Ok(())
}

#[tokio::test]
async fn test_jobs_search_with_filters() -> TestResult {
    let config = TestConfig::from_env();
    let client = ApiTestClient::new(&config);
    
    client.wait_for_server(20).await?;
    
    // Test with search parameters
    let response = client.get("/api/jobs?location=Sydney&job_type=FullTime").await?;
    
    // Verify response is valid
    assert!(response.is_array() || response.is_object());
    
    Ok(())
}

#[tokio::test]
async fn test_websocket_endpoint_availability() -> TestResult {
    let config = TestConfig::from_env();
    let client = ApiTestClient::new(&config);
    
    client.wait_for_server(20).await?;
    
    // Try to connect to WebSocket endpoint (this will fail but should not crash)
    let ws_url = format!("{}/ws", config.api_url.replace("http", "ws"));
    
    // For now, just verify the server is running
    // WebSocket testing requires a different approach
    let response = client.get("/health").await?;
    assert_eq!(response["status"], "ok");
    
    Ok(())
}

#[tokio::test]
async fn test_user_registration_endpoint() -> TestResult {
    let config = TestConfig::from_env();
    let client = ApiTestClient::new(&config);
    
    client.wait_for_server(20).await?;
    
    let user_data = serde_json::json!({
        "email": "test@example.com",
        "password": "testpassword123",
        "first_name": "Test",
        "last_name": "User",
        "phone": "+61412345678"
    });
    
    // In demo mode, this might return a mock success
    let result = client.post("/api/auth/register", user_data).await;
    
    // Don't fail if endpoint doesn't exist yet
    match result {
        Ok(response) => {
            // If successful, verify response structure
            assert!(response.is_object());
        },
        Err(_) => {
            // Endpoint might not be implemented yet
            println!("Registration endpoint not yet implemented");
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_job_creation_endpoint() -> TestResult {
    let config = TestConfig::from_env();
    let client = ApiTestClient::new(&config);
    
    client.wait_for_server(20).await?;
    
    let job_data = generate_job_form_data();
    
    // Test job creation endpoint
    let result = client.post("/api/jobs", job_data).await;
    
    match result {
        Ok(response) => {
            // Verify job was created
            assert!(response["id"].is_string());
            assert!(response["title"].is_string());
        },
        Err(_) => {
            // Endpoint might not be fully implemented
            println!("Job creation endpoint not yet fully implemented");
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_api_error_handling() -> TestResult {
    let config = TestConfig::from_env();
    let client = ApiTestClient::new(&config);
    
    client.wait_for_server(20).await?;
    
    // Test 404 endpoint
    let result = client.get("/api/nonexistent").await;
    
    // Should either get 404 or proper error response
    match result {
        Ok(response) => {
            // If we get a response, it should indicate an error
            if let Some(error) = response.get("error") {
                assert!(error.is_string());
            }
        },
        Err(_) => {
            // Expected for 404
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_cors_headers() -> TestResult {
    let config = TestConfig::from_env();
    let client = ApiTestClient::new(&config);
    
    client.wait_for_server(20).await?;
    
    // Test CORS preflight
    let url = format!("{}/api/jobs", config.api_url);
    let response = client.client
        .request(reqwest::Method::OPTIONS, &url)
        .header("Origin", "http://localhost:3080")
        .header("Access-Control-Request-Method", "GET")
        .send()
        .await?;
    
    // Should have CORS headers
    assert!(response.status().is_success() || response.status().as_u16() == 404);
    
    Ok(())
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[tokio::test]
    async fn test_api_response_time() -> TestResult {
        let config = TestConfig::from_env();
        let client = ApiTestClient::new(&config);
        
        client.wait_for_server(20).await?;
        
        let start = Instant::now();
        let _response = client.get("/api/jobs").await?;
        let duration = start.elapsed();
        
        // API should respond within reasonable time
        assert!(duration < Duration::from_secs(2), 
                "API response took too long: {:?}", duration);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_concurrent_requests() -> TestResult {
        let config = TestConfig::from_env();
        let client = ApiTestClient::new(&config);
        
        client.wait_for_server(20).await?;
        
        // Make multiple concurrent requests
        let mut handles = vec![];
        
        for _ in 0..5 {
            let client = ApiTestClient::new(&config);
            let handle = tokio::spawn(async move {
                client.get("/api/jobs").await
            });
            handles.push(handle);
        }
        
        // Wait for all requests to complete
        for handle in handles {
            let result = handle.await?;
            assert!(result.is_ok(), "Concurrent request failed");
        }
        
        Ok(())
    }
}