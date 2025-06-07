use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use shared::types::{Job, JobFilters, SearchResponse};
use shared::validation::ValidatedJobSearchRequest;
use crate::AppError;

/// Advanced search service with comprehensive filtering, ranking, and analytics
#[derive(Debug)]
pub struct SearchService {
    // In production, this would contain database connections, search indices, etc.
}

/// Search result with relevance scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredJob {
    pub job: Job,
    pub relevance_score: f64,
    pub match_reasons: Vec<String>,
    pub distance_km: Option<f64>,
}

/// Advanced search filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSearchFilters {
    // Text search
    pub query: Option<String>,
    pub title_keywords: Option<Vec<String>>,
    pub company_keywords: Option<Vec<String>>,
    pub description_keywords: Option<Vec<String>>,
    
    // Job characteristics
    pub job_types: Option<Vec<String>>,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub employment_types: Option<Vec<String>>,
    
    // Location filters
    pub states: Option<Vec<String>>,
    pub cities: Option<Vec<String>>,
    pub postcodes: Option<Vec<String>>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius_km: Option<i32>,
    
    // Date filters
    pub posted_after: Option<DateTime<Utc>>,
    pub posted_before: Option<DateTime<Utc>>,
    pub start_date_after: Option<DateTime<Utc>>,
    pub start_date_before: Option<DateTime<Utc>>,
    
    // Pharmacy-specific filters
    pub requires_ahpra: Option<bool>,
    pub requires_vaccination: Option<bool>,
    pub experience_level: Option<Vec<String>>, // ["entry", "mid", "senior", "manager"]
    pub pharmacy_type: Option<Vec<String>>, // ["community", "hospital", "clinical", "industrial"]
    pub specializations: Option<Vec<String>>,
    
    // Urgency and features
    pub is_urgent: Option<bool>,
    pub remote_possible: Option<bool>,
    pub featured_only: Option<bool>,
    
    // Sorting and pagination
    pub sort_by: Option<String>, // "relevance", "date", "salary", "distance"
    pub sort_order: Option<String>, // "asc", "desc"
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

/// Search analytics and insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAnalytics {
    pub total_results: i64,
    pub avg_salary: Option<f64>,
    pub salary_distribution: HashMap<String, i32>, // Salary ranges
    pub location_distribution: HashMap<String, i32>,
    pub job_type_distribution: HashMap<String, i32>,
    pub top_employers: Vec<(String, i32)>,
    pub search_suggestions: Vec<String>,
    pub related_searches: Vec<String>,
}

impl SearchService {
    pub fn new() -> Self {
        Self {}
    }

    /// Perform comprehensive job search with scoring and analytics
    pub async fn search_jobs(
        &self,
        filters: AdvancedSearchFilters,
    ) -> Result<(Vec<ScoredJob>, SearchAnalytics), AppError> {
        // In demo mode, return sample scored results
        let sample_jobs = self.get_sample_jobs().await?;
        let scored_jobs = self.score_and_rank_jobs(sample_jobs, &filters).await?;
        
        // Apply filters
        let filtered_jobs = self.apply_filters(scored_jobs, &filters).await?;
        
        // Generate analytics
        let analytics = self.generate_analytics(&filtered_jobs, &filters).await?;
        
        // Apply pagination
        let page = filters.page.unwrap_or(1);
        let limit = filters.limit.unwrap_or(20);
        let start = ((page - 1) * limit) as usize;
        let end = (start + limit as usize).min(filtered_jobs.len());
        
        let paginated_jobs = filtered_jobs[start..end].to_vec();
        
        Ok((paginated_jobs, analytics))
    }

    /// Score and rank jobs based on relevance
    async fn score_and_rank_jobs(
        &self,
        jobs: Vec<Job>,
        filters: &AdvancedSearchFilters,
    ) -> Result<Vec<ScoredJob>, AppError> {
        let mut scored_jobs = Vec::new();
        
        for job in jobs {
            let score = self.calculate_relevance_score(&job, filters).await?;
            let match_reasons = self.generate_match_reasons(&job, filters).await?;
            let distance = self.calculate_distance(&job, filters).await?;
            
            scored_jobs.push(ScoredJob {
                job,
                relevance_score: score,
                match_reasons,
                distance_km: distance,
            });
        }
        
        // Sort by relevance score (highest first)
        scored_jobs.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        
        Ok(scored_jobs)
    }

    /// Calculate relevance score for a job (0.0 to 1.0)
    async fn calculate_relevance_score(
        &self,
        job: &Job,
        filters: &AdvancedSearchFilters,
    ) -> Result<f64, AppError> {
        let mut score = 0.0;
        let mut factors = 0;
        
        // Text relevance scoring
        if let Some(query) = &filters.query {
            let title_match = self.text_similarity(&job.title, query);
            let desc_match = self.text_similarity(&job.description, query);
            let company_match = self.text_similarity(&job.pharmacy_name, query);
            
            score += (title_match * 0.5 + desc_match * 0.3 + company_match * 0.2);
            factors += 1;
        }
        
        // Salary match scoring
        if let Some(min_salary) = filters.salary_min {
            let job_salary = (job.hourly_rate * 40.0 * 52.0) as i32;
            let salary_score = if job_salary >= min_salary {
                1.0 - ((job_salary - min_salary) as f64 / 100000.0).min(1.0)
            } else {
                0.5
            };
            score += salary_score;
            factors += 1;
        }
        
        // Location proximity scoring
        if let (Some(lat), Some(lng)) = (filters.latitude, filters.longitude) {
            if let (Some(job_lat), Some(job_lng)) = (job.latitude, job.longitude) {
                let distance = self.haversine_distance(lat, lng, job_lat, job_lng);
                let radius = filters.radius_km.unwrap_or(50) as f64;
                let location_score = (1.0 - (distance / radius).min(1.0));
                score += location_score;
                factors += 1;
            }
        }
        
        // Urgency boost
        if job.is_urgent {
            score += 0.1;
        }
        
        // Recency boost (jobs posted recently get higher scores)
        let days_old = (Utc::now() - job.created_at).num_days() as f64;
        let recency_score = (1.0 - (days_old / 30.0).min(1.0)) * 0.2;
        score += recency_score;
        factors += 1;
        
        // Average the scores
        if factors > 0 {
            Ok(score / factors as f64)
        } else {
            Ok(0.5) // Default score
        }
    }

    /// Generate human-readable match reasons
    async fn generate_match_reasons(
        &self,
        job: &Job,
        filters: &AdvancedSearchFilters,
    ) -> Result<Vec<String>, AppError> {
        let mut reasons = Vec::new();
        
        if let Some(query) = &filters.query {
            if job.title.to_lowercase().contains(&query.to_lowercase()) {
                reasons.push(format!("Title matches '{}'", query));
            }
            if job.description.to_lowercase().contains(&query.to_lowercase()) {
                reasons.push(format!("Description mentions '{}'", query));
            }
        }
        
        if let Some(min_salary) = filters.salary_min {
            let job_salary = (job.hourly_rate * 40.0 * 52.0) as i32;
            if job_salary >= min_salary {
                reasons.push(format!("Salary meets minimum requirement (${})", min_salary));
            }
        }
        
        if job.is_urgent {
            reasons.push("Urgent hiring".to_string());
        }
        
        // Location match
        if let (Some(lat), Some(lng)) = (filters.latitude, filters.longitude) {
            if let (Some(job_lat), Some(job_lng)) = (job.latitude, job.longitude) {
                let distance = self.haversine_distance(lat, lng, job_lat, job_lng);
                if distance <= filters.radius_km.unwrap_or(50) as f64 {
                    reasons.push(format!("Within {:.1} km", distance));
                }
            }
        }
        
        Ok(reasons)
    }

    /// Apply filters to scored jobs
    async fn apply_filters(
        &self,
        jobs: Vec<ScoredJob>,
        filters: &AdvancedSearchFilters,
    ) -> Result<Vec<ScoredJob>, AppError> {
        let mut filtered = jobs;
        
        // Apply state filter
        if let Some(states) = &filters.states {
            filtered.retain(|scored_job| {
                states.iter().any(|state| 
                    scored_job.job.state.to_string().contains(state)
                )
            });
        }
        
        // Apply salary filters
        if let Some(min_salary) = filters.salary_min {
            filtered.retain(|scored_job| {
                (scored_job.job.hourly_rate * 40.0 * 52.0) as i32 >= min_salary
            });
        }
        
        if let Some(max_salary) = filters.salary_max {
            filtered.retain(|scored_job| {
                (scored_job.job.hourly_rate * 40.0 * 52.0) as i32 <= max_salary
            });
        }
        
        // Apply urgency filter
        if let Some(urgent_only) = filters.is_urgent {
            if urgent_only {
                filtered.retain(|scored_job| scored_job.job.is_urgent);
            }
        }
        
        // Apply date filters
        if let Some(posted_after) = filters.posted_after {
            filtered.retain(|scored_job| scored_job.job.created_at >= posted_after);
        }
        
        Ok(filtered)
    }

    /// Generate search analytics
    async fn generate_analytics(
        &self,
        jobs: &[ScoredJob],
        _filters: &AdvancedSearchFilters,
    ) -> Result<SearchAnalytics, AppError> {
        let total_results = jobs.len() as i64;
        
        // Calculate average salary
        let salaries: Vec<i32> = jobs.iter()
            .filter_map(|j| Some((j.job.hourly_rate * 40.0 * 52.0) as i32))
            .collect();
        let avg_salary = if !salaries.is_empty() {
            Some(salaries.iter().sum::<i32>() as f64 / salaries.len() as f64)
        } else {
            None
        };
        
        // Salary distribution
        let mut salary_distribution = HashMap::new();
        for job in jobs {
            let salary = (job.job.hourly_rate * 40.0 * 52.0) as i32;
            let range = match salary {
                    0..=60000 => "Under $60k",
                    60001..=80000 => "$60k-$80k",
                    80001..=100000 => "$80k-$100k",
                    100001..=120000 => "$100k-$120k",
                    _ => "Over $120k",
                };
                *salary_distribution.entry(range.to_string()).or_insert(0) += 1;
        }
        
        // Location distribution
        let mut location_distribution = HashMap::new();
        for job in jobs {
            let location = format!("{}, {}", job.job.suburb, job.job.state);
            let location_parts: Vec<&str> = location.split(',').collect();
            if let Some(state) = location_parts.last() {
                let state = state.trim();
                *location_distribution.entry(state.to_string()).or_insert(0) += 1;
            }
        }
        
        // Job type distribution (simplified)
        let mut job_type_distribution = HashMap::new();
        for job in jobs {
            *job_type_distribution.entry(job.job.job_type.to_string()).or_insert(0) += 1;
        }
        
        // Top employers
        let mut employer_counts = HashMap::new();
        for job in jobs {
            *employer_counts.entry(job.job.pharmacy_name.clone()).or_insert(0) += 1;
        }
        let mut top_employers: Vec<(String, i32)> = employer_counts.into_iter().collect();
        top_employers.sort_by(|a, b| b.1.cmp(&a.1));
        top_employers.truncate(5);
        
        Ok(SearchAnalytics {
            total_results,
            avg_salary,
            salary_distribution,
            location_distribution,
            job_type_distribution: job_type_distribution.into_iter().collect(),
            top_employers,
            search_suggestions: vec![
                "Pharmacist Sydney".to_string(),
                "Hospital pharmacy".to_string(),
                "Clinical pharmacist".to_string(),
                "Pharmacy manager".to_string(),
            ],
            related_searches: vec![
                "Part-time pharmacist".to_string(),
                "Locum positions".to_string(),
                "Graduate pharmacy".to_string(),
                "Pharmacy technician".to_string(),
            ],
        })
    }

    /// Calculate distance between user location and job location
    async fn calculate_distance(&self, job: &Job, filters: &AdvancedSearchFilters) -> Result<Option<f64>, AppError> {
        if let (Some(user_lat), Some(user_lng), Some(job_lat), Some(job_lng)) = 
            (filters.latitude, filters.longitude, job.latitude, job.longitude) {
            // Haversine formula for distance calculation
            let earth_radius = 6371.0; // Earth's radius in kilometres
            
            let lat1_rad = user_lat.to_radians();
            let lat2_rad = job_lat.to_radians();
            let delta_lat = (job_lat - user_lat).to_radians();
            let delta_lng = (job_lng - user_lng).to_radians();
            
            let a = (delta_lat / 2.0).sin().powi(2) +
                    lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
            let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
            
            Ok(Some(earth_radius * c))
        } else {
            Ok(None)
        }
    }

    /// Calculate text similarity (simplified implementation)
    fn text_similarity(&self, text1: &str, text2: &str) -> f64 {
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();
        
        // Simple keyword matching
        let keywords: Vec<&str> = text2_lower.split_whitespace().collect();
        let matches = keywords.iter()
            .filter(|keyword| text1_lower.contains(*keyword))
            .count();
        
        if keywords.is_empty() {
            0.0
        } else {
            matches as f64 / keywords.len() as f64
        }
    }

    /// Calculate distance between two points using Haversine formula
    fn haversine_distance(&self, lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
        let r = 6371.0; // Earth's radius in kilometers
        
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();
        let delta_lat = (lat2 - lat1).to_radians();
        let delta_lng = (lng2 - lng1).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) + 
                lat1_rad.cos() * lat2_rad.cos() * (delta_lng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        r * c
    }

    /// Get sample jobs for demo mode
    async fn get_sample_jobs(&self) -> Result<Vec<Job>, AppError> {
        // Return sample jobs from the job service
        let (jobs, _) = crate::services::JobService::list_jobs(
            shared::types::JobFilters::default(),
            Some(1),
            Some(50),
        ).await?;
        
        Ok(jobs)
    }

    /// Auto-complete search suggestions
    pub async fn get_search_suggestions(&self, query: &str) -> Result<Vec<String>, AppError> {
        // In production, this would query a search index or database
        let suggestions = vec![
            format!("{} pharmacist", query),
            format!("{} pharmacy", query),
            format!("{} clinical", query),
            format!("{} hospital", query),
            format!("{} community", query),
        ];
        
        Ok(suggestions.into_iter()
            .filter(|s| s.len() > query.len())
            .take(5)
            .collect())
    }

    /// Get trending searches
    pub async fn get_trending_searches(&self) -> Result<Vec<String>, AppError> {
        Ok(vec![
            "Clinical pharmacist Melbourne".to_string(),
            "Hospital pharmacy Sydney".to_string(),
            "Part-time pharmacist Brisbane".to_string(),
            "Pharmacy manager Perth".to_string(),
            "Locum pharmacist Adelaide".to_string(),
        ])
    }

    /// Save search for analytics and personalization
    pub async fn save_search(
        &self,
        user_id: Option<Uuid>,
        query: &str,
        filters: &AdvancedSearchFilters,
        results_count: i64,
    ) -> Result<(), AppError> {
        // In production, save to database for analytics
        tracing::info!(
            "Search saved: user_id={:?}, query={}, results={}", 
            user_id, query, results_count
        );
        Ok(())
    }
}

impl Default for SearchService {
    fn default() -> Self {
        Self::new()
    }
}