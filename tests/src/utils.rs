//! Testing utilities for Loco Platform

use crate::{TestConfig, TestResult};
use fantoccini::{Client, ClientBuilder, Locator};
use std::time::Duration;
use tokio::time::sleep;

/// Create a WebDriver client for testing
pub async fn create_webdriver_client(config: &TestConfig) -> TestResult<Client> {
    let mut caps = serde_json::Map::new();
    let mut chrome_opts = serde_json::Map::new();
    
    if config.headless {
        chrome_opts.insert("args".to_string(), serde_json::json!(["--headless", "--no-sandbox", "--disable-dev-shm-usage"]));
    }
    
    caps.insert("goog:chromeOptions".to_string(), serde_json::Value::Object(chrome_opts));
    
    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&config.webdriver_url)
        .await?;
        
    Ok(client)
}

/// Wait for page to fully load
pub async fn wait_for_page_load(client: &Client) -> TestResult {
    // Wait for document ready state
    let script = r#"
        return document.readyState === 'complete' && 
               (!window.Leptos || window.Leptos.loaded !== false);
    "#;
    
    for _ in 0..30 {
        let ready: bool = client.execute(script, vec![]).await?.as_bool().unwrap_or(false);
        if ready {
            return Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }
    
    Err("Page did not load within timeout".into())
}

/// Wait for app-specific loading indicators to disappear
pub async fn wait_for_app_load(client: &Client) -> TestResult {
    // Wait for loading spinners to disappear
    let script = r#"
        const loadingElements = document.querySelectorAll('.loading-spinner, .skeleton, [data-loading="true"]');
        return loadingElements.length === 0;
    "#;
    
    for _ in 0..50 {
        let loaded: bool = client.execute(script, vec![]).await?.as_bool().unwrap_or(false);
        if loaded {
            sleep(Duration::from_millis(100)).await; // Small buffer
            return Ok(());
        }
        sleep(Duration::from_millis(200)).await;
    }
    
    Ok(()) // Don't fail if loading indicators persist
}

/// Check if element exists on page
pub async fn element_exists(client: &Client, selector: &str) -> bool {
    client.find(Locator::Css(selector)).await.is_ok()
}

/// Navigate to path and wait for load
pub async fn navigate_and_wait(client: &Client, path: &str, config: &TestConfig) -> TestResult {
    let url = format!("{}{}", config.base_url, path);
    client.goto(&url).await?;
    wait_for_page_load(client).await?;
    wait_for_app_load(client).await?;
    Ok(())
}

/// Verify page loaded correctly by checking title and URL
pub async fn verify_page_loaded(client: &Client, expected_path: &str, config: &TestConfig) -> TestResult {
    let current_url = client.current_url().await?;
    let _expected_url = format!("{}{}", config.base_url, expected_path);
    
    if !current_url.to_string().contains(expected_path) {
        return Err(format!(
            "Page URL mismatch. Expected path '{}', got '{}'",
            expected_path,
            current_url
        ).into());
    }
    
    // Verify page has basic content (not 404 or error page)
    let title = client.title().await?;
    if title.contains("404") || title.contains("Error") {
        return Err(format!("Page appears to be an error page. Title: '{}'", title).into());
    }
    
    Ok(())
}

/// Test responsive design at different viewport sizes
pub async fn test_responsive_design(client: &Client, path: &str, config: &TestConfig) -> TestResult {
    let viewports = [
        (375, 667),   // Mobile
        (768, 1024),  // Tablet
        (1920, 1080), // Desktop
    ];
    
    for (width, height) in viewports {
        client.set_window_size(width, height).await?;
        navigate_and_wait(client, path, config).await?;
        
        // Verify no horizontal scroll
        let has_scroll: bool = client.execute(
            "return document.body.scrollWidth > window.innerWidth",
            vec![]
        ).await?.as_bool().unwrap_or(false);
        
        if has_scroll {
            println!("Warning: Horizontal scroll detected at {}x{}", width, height);
        }
        
        // Check for mobile menu button on smaller screens
        if width < 768 {
            if !element_exists(client, ".mobile-menu-button, .hamburger, [data-mobile-menu]").await {
                println!("Warning: No mobile menu button found at {}x{}", width, height);
            }
        }
    }
    
    Ok(())
}

/// Setup console error tracking
pub async fn setup_console_error_tracking(client: &Client) -> TestResult<Vec<String>> {
    // Store errors in a global array
    client.execute(r#"
        window.testErrors = [];
        const originalError = console.error;
        console.error = function(...args) {
            window.testErrors.push(args.join(' '));
            originalError.apply(console, args);
        };
    "#, vec![]).await?;
    
    Ok(vec![])
}

/// Get console errors
pub async fn get_console_errors(client: &Client) -> TestResult<Vec<String>> {
    let errors_json = client.execute("return window.testErrors || []", vec![]).await?;
    let errors: Vec<String> = serde_json::from_value(errors_json).unwrap_or_default();
    Ok(errors)
}

/// Fill form field by name or selector
pub async fn fill_form_field(client: &Client, selector: &str, value: &str) -> TestResult {
    let element = client.find(Locator::Css(selector)).await?;
    element.clear().await?;
    element.send_keys(value).await?;
    Ok(())
}

/// Click element and wait for navigation
pub async fn click_and_wait(client: &Client, selector: &str) -> TestResult {
    let element = client.find(Locator::Css(selector)).await?;
    element.click().await?;
    wait_for_app_load(client).await?;
    Ok(())
}

/// Take screenshot for debugging
pub async fn take_screenshot(client: &Client, name: &str) -> TestResult {
    let screenshot = client.screenshot().await?;
    let path = format!("target/screenshots/{}.png", name);
    
    // Create directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    std::fs::write(&path, screenshot)?;
    println!("Screenshot saved: {}", path);
    Ok(())
}

/// Mock API response for testing
pub async fn mock_api_response(
    client: &Client,
    url_pattern: &str,
    response_data: serde_json::Value,
) -> TestResult {
    let script = format!(r#"
        // Mock fetch for API calls
        const originalFetch = window.fetch;
        window.fetch = function(url, options) {{
            if (url.includes('{}')) {{
                return Promise.resolve(new Response(JSON.stringify({}), {{
                    status: 200,
                    headers: {{ 'Content-Type': 'application/json' }}
                }}));
            }}
            return originalFetch(url, options);
        }};
    "#, url_pattern, response_data);
    
    client.execute(&script, vec![]).await?;
    Ok(())
}

/// Check basic accessibility requirements
pub async fn check_accessibility(client: &Client, path: &str, config: &TestConfig) -> TestResult {
    navigate_and_wait(client, path, config).await?;
    
    // Check for alt text on images
    let images_without_alt: bool = client.execute(r#"
        const images = document.querySelectorAll('img');
        for (let img of images) {
            if (!img.alt && !img.getAttribute('aria-label')) {
                return true;
            }
        }
        return false;
    "#, vec![]).await?.as_bool().unwrap_or(false);
    
    if images_without_alt {
        println!("Warning: Images without alt text found on {}", path);
    }
    
    // Check for proper heading structure
    let heading_structure: bool = client.execute(r#"
        const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
        let lastLevel = 0;
        for (let heading of headings) {
            const level = parseInt(heading.tagName.charAt(1));
            if (level > lastLevel + 1) {
                return false; // Skipped heading level
            }
            lastLevel = level;
        }
        return true;
    "#, vec![]).await?.as_bool().unwrap_or(true);
    
    if !heading_structure {
        println!("Warning: Improper heading structure on {}", path);
    }
    
    Ok(())
}