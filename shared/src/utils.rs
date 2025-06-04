use crate::types::{Job, JobFilters};
use geo::{Point, HaversineDistance};

/// Calculate distance between two coordinates using Haversine formula
pub fn calculate_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let point1 = Point::new(lng1, lat1);
    let point2 = Point::new(lng2, lat2);
    point1.haversine_distance(&point2) / 1000.0 // Convert to kilometers
}

/// Filter jobs based on distance from user location
pub fn filter_jobs_by_distance(
    jobs: Vec<Job>,
    user_location: Option<(f64, f64)>,
    max_distance: Option<f64>,
) -> Vec<Job> {
    match (user_location, max_distance) {
        (Some((user_lat, user_lng)), Some(max_dist)) => {
            jobs.into_iter()
                .filter_map(|mut job| {
                    if let (Some(job_lat), Some(job_lng)) = (job.latitude, job.longitude) {
                        let distance = calculate_distance(user_lat, user_lng, job_lat, job_lng);
                        if distance <= max_dist {
                            job.distance_km = Some(distance);
                            Some(job)
                        } else {
                            None
                        }
                    } else {
                        Some(job)
                    }
                })
                .collect()
        }
        (Some((user_lat, user_lng)), None) => {
            jobs.into_iter()
                .map(|mut job| {
                    if let (Some(job_lat), Some(job_lng)) = (job.latitude, job.longitude) {
                        let distance = calculate_distance(user_lat, user_lng, job_lat, job_lng);
                        job.distance_km = Some(distance);
                    }
                    job
                })
                .collect()
        }
        _ => jobs,
    }
}

/// Apply filters to job list
pub fn apply_job_filters(jobs: Vec<Job>, filters: &JobFilters) -> Vec<Job> {
    jobs.into_iter()
        .filter(|job| {
            // Filter by job type
            if let Some(ref job_type) = filters.job_type {
                if &job.job_type != job_type {
                    return false;
                }
            }
            
            // Filter by hourly rate
            if let Some(min_rate) = filters.min_rate {
                if job.hourly_rate < min_rate {
                    return false;
                }
            }
            
            if let Some(max_rate) = filters.max_rate {
                if job.hourly_rate > max_rate {
                    return false;
                }
            }
            
            // Filter by suburb
            if let Some(ref suburb) = filters.suburb {
                if !job.suburb.to_lowercase().contains(&suburb.to_lowercase()) {
                    return false;
                }
            }
            
            // Filter by state
            if let Some(ref state) = filters.state {
                if job.state.to_string() != *state {
                    return false;
                }
            }
            
            // Filter by urgent status
            if let Some(is_urgent) = filters.is_urgent {
                if job.is_urgent != is_urgent {
                    return false;
                }
            }
            
            // Filter by date range
            if let Some(start_date) = filters.start_date {
                if job.start_date < start_date {
                    return false;
                }
            }
            
            if let Some(end_date) = filters.end_date {
                if job.end_date > end_date {
                    return false;
                }
            }
            
            true
        })
        .collect()
}

/// Format currency for Australian dollars
pub fn format_currency(amount: f64) -> String {
    format!("${:.0}/hr", amount)
}

/// Format date for Australian timezone
pub fn format_date_au(date: &chrono::DateTime<chrono::Utc>) -> String {
    date.format("%d/%m/%Y").to_string()
}

/// Validate Australian postcode
pub fn validate_postcode(postcode: &str) -> bool {
    postcode.len() == 4 && postcode.chars().all(|c| c.is_ascii_digit())
}

/// Validate Australian phone number
pub fn validate_phone_au(phone: &str) -> bool {
    let cleaned = phone.replace(&[' ', '-', '(', ')'][..], "");
    
    // Mobile: 04XX XXX XXX or +61 4XX XXX XXX
    // Landline: 0X XXXX XXXX or +61 X XXXX XXXX
    if cleaned.starts_with("+61") {
        let without_country = &cleaned[3..];
        (without_country.len() == 9 && without_country.starts_with('4')) ||
        (without_country.len() == 9 && !without_country.starts_with('4'))
    } else if cleaned.starts_with('0') {
        cleaned.len() == 10
    } else {
        false
    }
}