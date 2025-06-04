use serde::{Deserialize, Serialize};
use shared::{Job, User, JobFilters};
use fermi::Atom;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub user: Option<User>,
    pub jobs: Vec<Job>,
    pub filters: JobFilters,
    pub search_term: String,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub selected_job: Option<Job>,
    pub user_location: Option<(f64, f64)>, // (lat, lng)
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_user(&mut self, user: Option<User>) {
        self.user = user;
    }
    
    pub fn set_jobs(&mut self, jobs: Vec<Job>) {
        self.jobs = jobs;
    }
    
    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }
    
    pub fn set_error(&mut self, error: Option<String>) {
        self.error_message = error;
    }
    
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }
}

// Global state atoms for Fermi
pub static GLOBAL_STATE: Atom<AppState> = Atom(|_| AppState::default());
pub static USER_LOCATION: Atom<Option<(f64, f64)>> = Atom(|_| None);
pub static JOBS_LIST: Atom<Vec<Job>> = Atom(|_| Vec::new());
pub static SEARCH_FILTERS: Atom<JobFilters> = Atom(|_| JobFilters {
    job_type: None,
    min_rate: None,
    max_rate: None,
    suburb: None,
    state: None,
    is_urgent: None,
    max_distance: None,
    start_date: None,
    end_date: None,
});
pub static IS_LOADING: Atom<bool> = Atom(|_| false);
pub static ERROR_MESSAGE: Atom<Option<String>> = Atom(|_| None);