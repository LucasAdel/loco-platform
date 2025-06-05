use shared::{
    types::{Job, JobId},
    locations::{
        fix_job_location,
        fix_job_locations,
        job_location_needs_fixing,
        LocationFixStats,
    },
    AppError,
};
use tracing::{info, warn};

/// Service for managing job locations
pub struct LocationService;

impl LocationService {
    /// Fix a single job's location before saving
    pub fn ensure_valid_location(job: &mut Job) -> Result<(), AppError> {
        if job_location_needs_fixing(job) {
            let original_lat = job.latitude;
            let original_lng = job.longitude;
            
            fix_job_location(job);
            
            info!(
                "Fixed location for job '{}' in {}: ({:?}, {:?}) -> ({:?}, {:?})",
                job.title,
                job.suburb,
                original_lat,
                original_lng,
                job.latitude,
                job.longitude
            );
        }
        
        Ok(())
    }
    
    /// Fix multiple jobs' locations
    pub fn fix_job_locations_batch(jobs: &mut [Job]) -> LocationFixStats {
        let stats = shared::locations::fix_job_locations_with_stats(jobs);
        
        info!(
            "Location fix stats: {} total, {} fixed (missing: {}, invalid: {}, swapped: {})",
            stats.total_jobs,
            stats.missing_coordinates + stats.invalid_coordinates + stats.swapped_coordinates,
            stats.missing_coordinates,
            stats.invalid_coordinates,
            stats.swapped_coordinates
        );
        
        if stats.defaulted_to_cbd > 0 {
            warn!(
                "{} jobs defaulted to Adelaide CBD due to unknown suburbs",
                stats.defaulted_to_cbd
            );
        }
        
        stats
    }
    
    /// Validate that coordinates are within Adelaide metro area
    pub fn validate_adelaide_location(lat: f64, lng: f64) -> Result<(), AppError> {
        if !shared::locations::is_valid_adelaide_location(lat, lng) {
            return Err(AppError::validation(
                "location",
                "Coordinates must be within Adelaide metropolitan area"
            ));
        }
        Ok(())
    }
    
    /// Get suburb coordinates
    pub fn get_suburb_coordinates(suburb: &str) -> Option<(f64, f64)> {
        shared::locations::get_suburb_coordinates(suburb)
    }
    
    /// Get a list of all valid Adelaide suburbs
    pub fn get_valid_suburbs() -> Vec<&'static str> {
        use shared::locations::ADELAIDE_SUBURBS;
        let mut suburbs: Vec<&'static str> = ADELAIDE_SUBURBS.keys().copied().collect();
        suburbs.sort();
        suburbs
    }
    
    /// Calculate distance between two points (in kilometers)
    pub fn calculate_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
        use std::f64::consts::PI;
        
        let r = 6371.0; // Earth's radius in kilometers
        
        let lat1_rad = lat1 * PI / 180.0;
        let lat2_rad = lat2 * PI / 180.0;
        let delta_lat = (lat2 - lat1) * PI / 180.0;
        let delta_lng = (lng2 - lng1) * PI / 180.0;
        
        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
        
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        r * c
    }
    
    /// Find jobs within a certain radius of a location
    pub fn find_jobs_within_radius(
        jobs: &[Job],
        center_lat: f64,
        center_lng: f64,
        radius_km: f64,
    ) -> Vec<(Job, f64)> {
        let mut results: Vec<(Job, f64)> = jobs
            .iter()
            .filter_map(|job| {
                if let (Some(lat), Some(lng)) = (job.latitude, job.longitude) {
                    let distance = Self::calculate_distance(center_lat, center_lng, lat, lng);
                    if distance <= radius_km {
                        Some((job.clone(), distance))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by distance
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::types::{JobBuilder, Postcode, AustralianState};
    use uuid::Uuid;

    #[test]
    fn test_distance_calculation() {
        // Adelaide CBD to Norwood (should be about 3-4km)
        let distance = LocationService::calculate_distance(
            -34.9285, 138.6007, // Adelaide CBD
            -34.9206, 138.6326, // Norwood
        );
        assert!(distance > 2.0 && distance < 5.0);
    }

    #[test]
    fn test_find_jobs_within_radius() {
        let jobs = vec![
            create_test_job("Norwood", -34.9206, 138.6326),
            create_test_job("Mount Barker", -35.0667, 138.8667), // Far away
            create_test_job("North Adelaide", -34.9065, 138.5934),
        ];
        
        // Search from Adelaide CBD with 5km radius
        let nearby = LocationService::find_jobs_within_radius(
            &jobs,
            -34.9285,
            138.6007,
            5.0,
        );
        
        assert_eq!(nearby.len(), 2); // Should find Norwood and North Adelaide
        assert_eq!(nearby[0].0.suburb, "North Adelaide"); // Closer one first
        assert_eq!(nearby[1].0.suburb, "Norwood");
    }
    
    fn create_test_job(suburb: &str, lat: f64, lng: f64) -> Job {
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Test Job".to_string(),
            description: "Test".to_string(),
            pharmacy_name: "Test Pharmacy".to_string(),
            hourly_rate: 50.0,
            address: "123 Test St".to_string(),
            suburb: suburb.to_string(),
            postcode: Postcode::new("5000").unwrap(),
            state: AustralianState::SA,
            latitude: Some(lat),
            longitude: Some(lng),
            start_date: chrono::Utc::now(),
            end_date: chrono::Utc::now(),
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            job_type: shared::types::JobType::FullTime,
            status: shared::types::JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            created_by: shared::types::UserId(Uuid::new_v4()),
        }
    }
}