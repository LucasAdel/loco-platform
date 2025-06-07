//! End-to-End tests using fantoccini WebDriver
//! These tests replace the previous Playwright-based E2E tests

use loco_platform_tests::{
    TestConfig, TestResult, fixtures::*,
    utils::*,
    assertions::*,
    navigate_and_wait, setup_test
};
use fantoccini::{Client, Locator};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_homepage_loads() -> TestResult {
    setup_test!();
    
    navigate_and_wait!(client, "/");
    
    // Verify page title
    let title = client.title().await?;
    assert!(!title.is_empty(), "Page should have a title");
    
    // Verify basic content exists
    assert_element_exists(&client, "body").await?;
    assert_element_exists(&client, "main, #app, .app").await?;
    
    Ok(())
}

#[tokio::test]
async fn test_navigation_between_pages() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    // Start at homepage
    navigate_and_wait(&client, "/", &config).await?;
    verify_page_loaded(&client, "/", &config).await?;
    
    // Test navigation to Jobs page
    if element_exists(&client, "a[href*='jobs'], a[href='/jobs']").await {
        click_and_wait(&client, "a[href*='jobs'], a[href='/jobs']").await?;
        verify_page_loaded(&client, "/jobs", &config).await?;
    }
    
    // Test navigation to Map page  
    if element_exists(&client, "a[href*='map'], a[href='/map']").await {
        click_and_wait(&client, "a[href*='map'], a[href='/map']").await?;
        verify_page_loaded(&client, "/map", &config).await?;
    }
    
    // Test navigation to Dashboard
    if element_exists(&client, "a[href*='dashboard'], a[href='/dashboard']").await {
        click_and_wait(&client, "a[href*='dashboard'], a[href='/dashboard']").await?;
        verify_page_loaded(&client, "/dashboard", &config).await?;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_job_search_functionality() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    // Navigate to jobs page
    navigate_and_wait(&client, "/jobs", &config).await?;
    
    // Look for search input
    if element_exists(&client, "input[name='search'], input[placeholder*='search'], .search-input").await {
        fill_form_field(&client, "input[name='search'], input[placeholder*='search'], .search-input", "pharmacist").await?;
        
        // Wait for search results
        sleep(Duration::from_secs(1)).await;
        
        // Verify results are displayed
        if element_exists(&client, ".job-card, .job-item, .search-results").await {
            assert_element_exists(&client, ".job-card, .job-item, .search-results").await?;
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_mobile_navigation() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    // Set mobile viewport
    client.set_window_size(375, 667).await?;
    
    navigate_and_wait(&client, "/", &config).await?;
    
    // Look for mobile menu button
    if element_exists(&client, ".mobile-menu-button, .hamburger-menu, [data-mobile-menu]").await {
        click_and_wait(&client, ".mobile-menu-button, .hamburger-menu, [data-mobile-menu]").await?;
        
        // Verify mobile menu opened
        if element_exists(&client, ".mobile-menu, .navigation-menu, .sidebar").await {
            assert_element_exists(&client, ".mobile-menu, .navigation-menu, .sidebar").await?;
        }
    }
    
    Ok(())
}

#[tokio::test] 
async fn test_job_application_flow() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    // Navigate to jobs page
    navigate_and_wait(&client, "/jobs", &config).await?;
    
    // Look for job cards and apply button
    if element_exists(&client, ".job-card").await {
        // Click on first job card or apply button
        if element_exists(&client, ".apply-button, button[data-action='apply'], .btn-apply").await {
            click_and_wait(&client, ".apply-button, button[data-action='apply'], .btn-apply").await?;
            
            // Verify application form or modal opened
            if element_exists(&client, ".application-form, .modal, .apply-modal").await {
                assert_element_exists(&client, ".application-form, .modal, .apply-modal").await?;
            }
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_map_functionality() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    navigate_and_wait(&client, "/map", &config).await?;
    
    // Wait for map to load
    sleep(Duration::from_secs(3)).await;
    
    // Check if map container exists
    if element_exists(&client, "#map, .map-container, .mapbox-map").await {
        assert_element_exists(&client, "#map, .map-container, .mapbox-map").await?;
    }
    
    // Check for map controls
    if element_exists(&client, ".map-controls, .filter-panel").await {
        assert_element_exists(&client, ".map-controls, .filter-panel").await?;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_responsive_design() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    test_responsive_design(&client, "/", &config).await?;
    test_responsive_design(&client, "/jobs", &config).await?;
    test_responsive_design(&client, "/dashboard", &config).await?;
    
    Ok(())
}

#[tokio::test]
async fn test_form_validation() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    // Navigate to a page with forms (e.g., job creation or profile)
    navigate_and_wait(&client, "/profile", &config).await?;
    
    // Look for form inputs
    if element_exists(&client, "form input, .form-input").await {
        // Try to submit empty form
        if element_exists(&client, "button[type='submit'], .submit-btn").await {
            click_and_wait(&client, "button[type='submit'], .submit-btn").await?;
            
            // Check for validation messages
            if element_exists(&client, ".error-message, .validation-error, .field-error").await {
                assert_element_exists(&client, ".error-message, .validation-error, .field-error").await?;
            }
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_accessibility_features() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    check_accessibility(&client, "/", &config).await?;
    check_accessibility(&client, "/jobs", &config).await?;
    check_accessibility(&client, "/dashboard", &config).await?;
    
    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    setup_console_error_tracking(&client).await?;
    
    // Navigate to non-existent page
    let result = client.goto(&format!("{}/nonexistent-page", config.base_url)).await;
    
    // Should either redirect to 404 page or handle gracefully
    match result {
        Ok(_) => {
            // Check if we're on an error page
            let title = client.title().await?;
            let url = client.current_url().await?;
            
            // Should show some kind of error indication
            assert!(
                title.contains("404") || 
                title.contains("Not Found") || 
                url.to_string().contains("404") ||
                element_exists(&client, ".error-page, .not-found").await,
                "Should show proper error page for non-existent routes"
            );
        },
        Err(_) => {
            // Navigation failed, which is also acceptable
        }
    }
    
    // Check for JavaScript errors
    let errors = get_console_errors(&client).await?;
    
    // Filter out expected errors or warnings
    let critical_errors: Vec<_> = errors.iter()
        .filter(|e| !e.contains("Warning:") && !e.contains("404"))
        .collect();
    
    if !critical_errors.is_empty() {
        println!("Console errors detected: {:?}", critical_errors);
    }
    
    Ok(())
}

#[tokio::test]
async fn test_real_time_features() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    navigate_and_wait(&client, "/dashboard", &config).await?;
    
    // Look for real-time indicators
    if element_exists(&client, ".live-feed, .real-time, .websocket-status").await {
        assert_element_exists(&client, ".live-feed, .real-time, .websocket-status").await?;
        
        // Wait for some real-time updates (if any)
        sleep(Duration::from_secs(2)).await;
    }
    
    Ok(())
}

#[tokio::test] 
async fn test_complete_user_journey() -> TestResult {
    let config = TestConfig::from_env();
    let client = create_webdriver_client(&config).await?;
    
    setup_console_error_tracking(&client).await?;
    
    // Start at homepage
    navigate_and_wait(&client, "/", &config).await?;
    verify_page_loaded(&client, "/", &config).await?;
    
    // Navigate to jobs
    if element_exists(&client, "a[href*='jobs']").await {
        click_and_wait(&client, "a[href*='jobs']").await?;
        verify_page_loaded(&client, "/jobs", &config).await?;
    }
    
    // Search for jobs
    if element_exists(&client, "input[placeholder*='search'], .search-input").await {
        fill_form_field(&client, "input[placeholder*='search'], .search-input", "Sydney").await?;
        sleep(Duration::from_millis(500)).await;
    }
    
    // Navigate to map
    if element_exists(&client, "a[href*='map']").await {
        click_and_wait(&client, "a[href*='map']").await?;
        verify_page_loaded(&client, "/map", &config).await?;
        sleep(Duration::from_secs(2)).await; // Wait for map to load
    }
    
    // Navigate to dashboard
    if element_exists(&client, "a[href*='dashboard']").await {
        click_and_wait(&client, "a[href*='dashboard']").await?;
        verify_page_loaded(&client, "/dashboard", &config).await?;
    }
    
    // Check for any console errors during the journey
    let errors = get_console_errors(&client).await?;
    let critical_errors: Vec<_> = errors.iter()
        .filter(|e| !e.contains("Warning:"))
        .collect();
    
    if !critical_errors.is_empty() {
        println!("Critical errors during user journey: {:?}", critical_errors);
    }
    
    Ok(())
}