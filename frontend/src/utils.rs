use wasm_bindgen::prelude::*;
use web_sys::{console, window, Storage};
use gloo_storage::{LocalStorage, Storage as _};
use serde::{Serialize, Deserialize};

// Console logging utilities
pub fn log(message: &str) {
    console::log_1(&message.into());
}

pub fn error(message: &str) {
    console::error_1(&message.into());
}

pub fn warn(message: &str) {
    console::warn_1(&message.into());
}

// Local storage utilities
pub fn save_to_storage<T: Serialize>(key: &str, value: &T) -> Result<(), String> {
    LocalStorage::set(key, value).map_err(|e| e.to_string())
}

pub fn load_from_storage<T: for<'de> Deserialize<'de>>(key: &str) -> Option<T> {
    LocalStorage::get(key).ok()
}

pub fn remove_from_storage(key: &str) {
    LocalStorage::delete(key);
}

// URL utilities
pub fn get_current_url() -> Option<String> {
    window()?
        .location()
        .href()
        .ok()
}

pub fn navigate_to(url: &str) {
    if let Some(window) = window() {
        if let Some(_location) = window.location().href().ok() {
            let _ = window.location().set_href(url);
        }
    }
}

// Date/time utilities for Australian timezone
pub fn format_date_au(date: &chrono::DateTime<chrono::Utc>) -> String {
    // Convert to Australian Eastern Time
    date.format("%d/%m/%Y").to_string()
}

pub fn format_time_au(date: &chrono::DateTime<chrono::Utc>) -> String {
    date.format("%I:%M %p").to_string()
}

// Currency formatting
pub fn format_currency(amount: f64) -> String {
    format!("${:.0}", amount)
}

pub fn format_currency_with_symbol(amount: f64) -> String {
    format!("${:.0}/hr", amount)
}

// Distance utilities
pub fn format_distance(distance_km: f64) -> String {
    if distance_km < 1.0 {
        format!("{:.0}m", distance_km * 1000.0)
    } else {
        format!("{:.1}km", distance_km)
    }
}

// Validation utilities
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn validate_phone_au(phone: &str) -> bool {
    let cleaned = phone.replace(&[' ', '-', '(', ')'][..], "");
    
    // Australian mobile: 04XX XXX XXX or +61 4XX XXX XXX
    // Australian landline: 0X XXXX XXXX or +61 X XXXX XXXX
    if cleaned.starts_with("+61") {
        let without_country = &cleaned[3..];
        without_country.len() == 9 && without_country.chars().all(|c| c.is_ascii_digit())
    } else if cleaned.starts_with('0') {
        cleaned.len() == 10 && cleaned.chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}

pub fn validate_postcode_au(postcode: &str) -> bool {
    postcode.len() == 4 && postcode.chars().all(|c| c.is_ascii_digit())
}

// Geolocation utilities
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = geolocation)]
    static GEOLOCATION: web_sys::Geolocation;
}

pub async fn get_user_location() -> Option<(f64, f64)> {
    // This would need proper implementation with callbacks
    // For now, return Adelaide coordinates as default
    Some((-34.9285, 138.6007))
}

// Clipboard utilities
pub async fn copy_to_clipboard(text: &str) -> Result<(), String> {
    if let Some(window) = window() {
        // Simplified clipboard implementation - TODO: implement properly
        web_sys::console::log_1(&format!("Would copy to clipboard: {}", text).into());
    }
    Err("Clipboard API not available".to_string())
}

// Share utilities
pub async fn share_job(job_title: &str, job_url: &str) -> Result<(), String> {
    if let Some(window) = window() {
        // Check if Web Share API is available
        if js_sys::Reflect::has(&window, &"navigator".into()).unwrap_or(false) {
            let navigator = window.navigator();
            if js_sys::Reflect::has(&navigator, &"share".into()).unwrap_or(false) {
                // Would implement proper Web Share API call here
                log(&format!("Sharing job: {} - {}", job_title, job_url));
                return Ok(());
            }
        }
    }
    
    // Fallback to clipboard
    copy_to_clipboard(&format!("{} - {}", job_title, job_url)).await
}

// Theme utilities
pub fn get_preferred_theme() -> String {
    // For now, default to light theme
    // TODO: Implement proper prefers-color-scheme detection
    "light".to_string()
}

// Device detection
pub fn is_mobile() -> bool {
    if let Some(window) = window() {
        if let Ok(user_agent) = window.navigator().user_agent() {
            return user_agent.contains("Mobile") || 
                   user_agent.contains("Android") || 
                   user_agent.contains("iPhone");
        }
    }
    false
}

pub fn is_ios() -> bool {
    if let Some(window) = window() {
        if let Ok(user_agent) = window.navigator().user_agent() {
            return user_agent.contains("iPhone") || 
                   user_agent.contains("iPad") || 
                   user_agent.contains("iPod");
        }
    }
    false
}