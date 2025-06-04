use dioxus::prelude::*;
use shared::types::{Job, SearchRequest};
use crate::services::ApiClient;

pub fn use_jobs() -> Signal<Vec<Job>> {
    use_signal(|| vec![])
}

pub fn use_job_search() -> Signal<SearchRequest> {
    use_signal(|| SearchRequest {
        query: Some(String::new()),
        filters: shared::types::JobFilters::default(),
        user_location: None,
        page: Some(1),
        limit: Some(20),
    })
}

pub fn use_geolocation() -> Signal<Option<(f64, f64)>> {
    let location = use_signal(|| Some((-34.9285, 138.6007))); // Default Adelaide location
    location
}

pub fn use_api_client() -> Signal<ApiClient> {
    use_signal(|| ApiClient::new())
}

pub fn use_loading() -> Signal<bool> {
    use_signal(|| false)
}

pub fn use_error() -> Signal<Option<String>> {
    use_signal(|| None)
}