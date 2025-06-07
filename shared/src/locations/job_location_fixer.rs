use crate::types::Job;
use super::suburb_coordinates::{
    get_location_with_fallback,
    is_valid_adelaide_location,
    fix_swapped_coordinates,
};

/// Fix location for a single job
pub fn fix_job_location(job: &mut Job) {
    // Get the best coordinates using our fallback logic
    let (lat, lng) = get_location_with_fallback(
        job.latitude,
        job.longitude,
        Some(&job.suburb),
        Some(&job.postcode.to_string()),
    );
    
    // Update the job's coordinates
    job.latitude = Some(lat);
    job.longitude = Some(lng);
}

/// Fix locations for a collection of jobs
pub fn fix_job_locations(jobs: &mut [Job]) {
    for job in jobs {
        fix_job_location(job);
    }
}

/// Check if a job's location needs fixing
pub fn job_location_needs_fixing(job: &Job) -> bool {
    match (job.latitude, job.longitude) {
        (Some(lat), Some(lng)) => {
            // Check if coordinates are valid for Adelaide
            !is_valid_adelaide_location(lat, lng)
        }
        // Missing coordinates definitely need fixing
        _ => true,
    }
}

/// Get a debug string describing the location fix applied
pub fn describe_location_fix(job: &Job, original_lat: Option<f64>, original_lng: Option<f64>) -> String {
    match (original_lat, original_lng, job.latitude, job.longitude) {
        (None, None, Some(new_lat), Some(new_lng)) => {
            format!(
                "Added coordinates for {} from suburb database: ({:.4}, {:.4})",
                job.suburb, new_lat, new_lng
            )
        }
        (Some(old_lat), Some(old_lng), Some(new_lat), Some(new_lng)) 
            if (old_lat - new_lat).abs() > 0.001 || (old_lng - new_lng).abs() > 0.001 => {
            format!(
                "Fixed invalid coordinates for {}: ({:.4}, {:.4}) -> ({:.4}, {:.4})",
                job.suburb, old_lat, old_lng, new_lat, new_lng
            )
        }
        _ => format!("No location fix needed for {}", job.suburb),
    }
}

/// Struct to hold location fix statistics
#[derive(Debug, Default)]
pub struct LocationFixStats {
    pub total_jobs: usize,
    pub missing_coordinates: usize,
    pub invalid_coordinates: usize,
    pub swapped_coordinates: usize,
    pub fixed_from_suburb: usize,
    pub fixed_from_postcode: usize,
    pub defaulted_to_cbd: usize,
}

/// Fix locations for a collection of jobs and return statistics
pub fn fix_job_locations_with_stats(jobs: &mut [Job]) -> LocationFixStats {
    let mut stats = LocationFixStats {
        total_jobs: jobs.len(),
        ..Default::default()
    };
    
    for job in jobs {
        let original_lat = job.latitude;
        let original_lng = job.longitude;
        
        // Check what kind of fix is needed
        match (original_lat, original_lng) {
            (None, None) => stats.missing_coordinates += 1,
            (Some(lat), Some(lng)) => {
                if !is_valid_adelaide_location(lat, lng) {
                    stats.invalid_coordinates += 1;
                    
                    // Check if coordinates are swapped
                    let (fixed_lat, fixed_lng) = fix_swapped_coordinates(lat, lng);
                    if (fixed_lat - lat).abs() > 0.001 || (fixed_lng - lng).abs() > 0.001 {
                        stats.swapped_coordinates += 1;
                    }
                }
            }
            _ => {} // Partially missing coordinates
        }
        
        // Apply the fix
        fix_job_location(job);
        
        // Track how the fix was made
        if job.latitude.is_some() && job.longitude.is_some() {
            match (original_lat, original_lng) {
                (None, None) | (Some(_), None) | (None, Some(_)) => {
                    // Coordinates were added from suburb/postcode
                    if job.suburb.to_lowercase() != "adelaide" {
                        stats.fixed_from_suburb += 1;
                    } else {
                        stats.defaulted_to_cbd += 1;
                    }
                }
                (Some(lat), Some(lng)) if !is_valid_adelaide_location(lat, lng) => {
                    // Invalid coordinates were replaced
                    stats.fixed_from_suburb += 1;
                }
                _ => {} // No fix needed
            }
        }
    }
    
    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{JobBuilder, Postcode, AustralianState};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_job(suburb: &str, postcode: &str, lat: Option<f64>, lng: Option<f64>) -> Job {
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Test Job".to_string(),
            description: "Test description".to_string(),
            pharmacy_name: "Test Pharmacy".to_string(),
            hourly_rate: 50.0,
            address: "123 Test St".to_string(),
            suburb: suburb.to_string(),
            postcode: Postcode::new(postcode).unwrap(),
            state: AustralianState::SA,
            latitude: lat,
            longitude: lng,
            start_date: Utc::now(),
            end_date: Utc::now(),
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            job_type: crate::types::JobType::FullTime,
            status: crate::types::JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: crate::types::UserId(Uuid::new_v4()),
        }
    }

    #[test]
    fn test_fix_missing_coordinates() {
        let mut job = create_test_job("Norwood", "5067", None, None);
        fix_job_location(&mut job);
        
        assert!(job.latitude.is_some());
        assert!(job.longitude.is_some());
        assert_eq!(job.latitude.unwrap(), -34.9206);
        assert_eq!(job.longitude.unwrap(), 138.6326);
    }

    #[test]
    fn test_fix_swapped_coordinates() {
        let mut job = create_test_job("Adelaide", "5000", Some(138.6007), Some(-34.9285));
        fix_job_location(&mut job);
        
        assert_eq!(job.latitude.unwrap(), -34.9285);
        assert_eq!(job.longitude.unwrap(), 138.6007);
    }

    #[test]
    fn test_preserve_valid_coordinates() {
        let mut job = create_test_job("Norwood", "5067", Some(-34.9206), Some(138.6326));
        fix_job_location(&mut job);
        
        // Should keep the original valid coordinates
        assert_eq!(job.latitude.unwrap(), -34.9206);
        assert_eq!(job.longitude.unwrap(), 138.6326);
    }
}