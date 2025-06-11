// Utility functions for the Loco Platform

use chrono::{DateTime, Local, Utc};
use leptos::*;

/// Format a date for display
pub fn format_date(date: DateTime<Utc>) -> String {
    date.with_timezone(&Local).format("%d %B %Y").to_string()
}

/// Format a date with time
pub fn format_datetime(date: DateTime<Utc>) -> String {
    date.with_timezone(&Local).format("%d %B %Y at %I:%M %p").to_string()
}

/// Format currency in AUD
pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

/// Format salary range
pub fn format_salary_range(min: Option<f64>, max: Option<f64>) -> String {
    match (min, max) {
        (Some(min), Some(max)) => format!("{} - {}", format_currency(min), format_currency(max)),
        (Some(min), None) => format!("{} +", format_currency(min)),
        (None, Some(max)) => format!("Up to {}", format_currency(max)),
        (None, None) => "Competitive".to_string(),
    }
}

/// Get initials from a name
pub fn get_initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

/// Generate a random color for avatars
pub fn get_avatar_color(name: &str) -> String {
    let colors = vec![
        "bg-blue-500",
        "bg-green-500",
        "bg-purple-500",
        "bg-pink-500",
        "bg-indigo-500",
        "bg-teal-500",
    ];
    
    let hash = name.chars().map(|c| c as usize).sum::<usize>();
    colors[hash % colors.len()].to_string()
}

/// Local storage helpers
pub mod storage {
    use web_sys::window;
    
    pub fn get_item(key: &str) -> Option<String> {
        window()?
            .local_storage()
            .ok()??
            .get_item(key)
            .ok()?
    }
    
    pub fn set_item(key: &str, value: &str) -> Result<(), String> {
        window()
            .ok_or("No window")?
            .local_storage()
            .map_err(|_| "No local storage")?
            .ok_or("No local storage")?
            .set_item(key, value)
            .map_err(|_| "Failed to set item".to_string())
    }
    
    pub fn remove_item(key: &str) -> Result<(), String> {
        window()
            .ok_or("No window")?
            .local_storage()
            .map_err(|_| "No local storage")?
            .ok_or("No local storage")?
            .remove_item(key)
            .map_err(|_| "Failed to remove item".to_string())
    }
}