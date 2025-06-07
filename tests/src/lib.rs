//! Loco Platform Testing Suite
//! 
//! This crate provides Rust-native testing utilities for the Loco Platform,
//! replacing the previous Playwright-based testing infrastructure.

use std::time::Duration;

pub mod utils;
pub mod fixtures;

// Re-export commonly used testing utilities
pub use utils::*;
pub use fixtures::*;

/// Configuration for test environments
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub base_url: String,
    pub api_url: String,
    pub webdriver_url: String,
    pub timeout: Duration,
    pub headless: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3080".to_string(),
            api_url: "http://localhost:3070".to_string(),
            webdriver_url: "http://localhost:4444".to_string(),
            timeout: Duration::from_secs(30),
            headless: true,
        }
    }
}

impl TestConfig {
    /// Create test config from environment variables
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("TEST_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3080".to_string()),
            api_url: std::env::var("TEST_API_URL")
                .unwrap_or_else(|_| "http://localhost:3070".to_string()),
            webdriver_url: std::env::var("WEBDRIVER_URL")
                .unwrap_or_else(|_| "http://localhost:4444".to_string()),
            timeout: Duration::from_secs(
                std::env::var("TEST_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30)
            ),
            headless: std::env::var("TEST_HEADLESS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        }
    }
}

/// Test result type for consistent error handling
pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Common test assertions
pub mod assertions {
    use super::*;
    
    /// Assert that an element exists on the page
    pub async fn assert_element_exists(
        client: &fantoccini::Client,
        selector: &str,
    ) -> TestResult {
        let element = client.find(fantoccini::Locator::Css(selector)).await?;
        assert!(!element.text().await?.is_empty(), "Element found but has no text: {}", selector);
        Ok(())
    }
    
    /// Assert that text is visible on the page
    pub async fn assert_text_visible(
        client: &fantoccini::Client,
        text: &str,
    ) -> TestResult {
        let element = client.find(fantoccini::Locator::XPath(&format!("//*[contains(text(), '{}')]", text))).await?;
        assert!(element.is_displayed().await?, "Text '{}' is not visible", text);
        Ok(())
    }
    
    /// Assert that a URL matches expected pattern
    pub fn assert_url_matches(actual: &str, expected_pattern: &str) -> TestResult {
        assert!(
            actual.contains(expected_pattern),
            "URL '{}' does not match pattern '{}'",
            actual,
            expected_pattern
        );
        Ok(())
    }
}

/// Macros for common test patterns
#[macro_export]
macro_rules! setup_test {
    () => {
        let config = $crate::TestConfig::from_env();
        let client = $crate::utils::create_webdriver_client(&config).await?;
    };
}

#[macro_export]
macro_rules! navigate_and_wait {
    ($client:expr, $path:expr) => {
        $client.goto(&format!("{}{}", $crate::TestConfig::from_env().base_url, $path)).await?;
        $crate::utils::wait_for_page_load(&$client).await?;
    };
}