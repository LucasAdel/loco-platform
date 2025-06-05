use chrono::{DateTime, Utc};

/// Format a date for display
pub fn format_date(date: DateTime<Utc>) -> String {
    date.format("%d %B %Y").to_string()
}

/// Format a date as relative time (e.g., "2 days ago")
pub fn format_relative_date(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(date);
    
    if duration.num_days() > 30 {
        format!("{} months ago", duration.num_days() / 30)
    } else if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else {
        "Just now".to_string()
    }
}

/// Calculate distance between two coordinates using Haversine formula
pub fn calculate_distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin() +
            lat1_rad.cos() * lat2_rad.cos() *
            (delta_lon / 2.0).sin() * (delta_lon / 2.0).sin();
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_KM * c
}

/// Format distance for display
pub fn format_distance(km: f64) -> String {
    if km < 1.0 {
        format!("{} m", (km * 1000.0).round() as i32)
    } else if km < 10.0 {
        format!("{:.1} km", km)
    } else {
        format!("{} km", km.round() as i32)
    }
}

/// Generate a random colour for avatars
pub fn get_avatar_colour(seed: &str) -> &'static str {
    let colours = [
        "bg-blue-500",
        "bg-green-500",
        "bg-yellow-500",
        "bg-red-500",
        "bg-purple-500",
        "bg-pink-500",
        "bg-indigo-500",
    ];
    
    let hash = seed.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
    colours[(hash % colours.len() as u32) as usize]
}

/// Get initials from a name
pub fn get_initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

/// Validate Australian postcode
pub fn is_valid_postcode(postcode: &str) -> bool {
    postcode.len() == 4 && postcode.chars().all(|c| c.is_ascii_digit())
}

/// Validate Australian phone number
pub fn is_valid_phone(phone: &str) -> bool {
    let cleaned = phone.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    cleaned.len() == 10 && (cleaned.starts_with("04") || cleaned.starts_with("02") || 
                            cleaned.starts_with("03") || cleaned.starts_with("07") || 
                            cleaned.starts_with("08"))
}