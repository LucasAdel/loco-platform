use dioxus::prelude::*;
use shared::types::{Job, JobType, JobId, UserId, Postcode, AustralianState, JobStatus};
use chrono::Utc;
use web_sys::{console, window, UrlSearchParams};
use gloo_timers::future::TimeoutFuture;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// ============================================================================
// Advanced Data Structures for Comprehensive Map Features
// ============================================================================

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct LocationData {
    coordinates: (f64, f64),
    accuracy: Option<f64>,
    source: LocationSource,
    timestamp: chrono::DateTime<chrono::Utc>,
    address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum LocationSource {
    GPS,
    Profile,
    Cached,
    Default,
}

#[derive(Clone, Debug, PartialEq)]
struct CommuteInfo {
    travel_time: f64,
    distance: f64,
    mode: TransportMode,
    cost: Option<f64>,
    route_points: Vec<(f64, f64)>,
}

#[derive(Clone, Debug, PartialEq)]
enum TransportMode {
    Walking,
    Driving,
    PublicTransport,
    Cycling,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatmapPoint {
    lat: f64,
    lng: f64,
    intensity: f64,
    data_type: HeatmapType,
}

#[derive(Clone, Debug, PartialEq)]
enum HeatmapType {
    Density,
    Salary,
    TimeBasedMorning,
    TimeBasedAfternoon,
    TimeBasedEvening,
}

#[derive(Clone, Debug, PartialEq)]
enum MapStyle {
    Light,
    Streets,
    Satellite,
    Dark,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct JobFilters {
    search_query: String,
    show_urgent_only: bool,
    show_hospital_jobs: bool,
    show_retail_jobs: bool,
    show_clinical_jobs: bool,
    show_locum_jobs: bool,
    show_compounding_jobs: bool,
    show_aged_care_jobs: bool,
    salary_range: (f64, f64),
    time_filter: TimeFilter,
    sort_by: SortOption,
    location_radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum TimeFilter {
    All,
    Morning,
    Afternoon,
    Evening,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum SortOption {
    Distance,
    Date,
    Rate,
    Relevance,
}

impl Default for JobFilters {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            show_urgent_only: false,
            show_hospital_jobs: true,
            show_retail_jobs: true,
            show_clinical_jobs: true,
            show_locum_jobs: true,
            show_compounding_jobs: true,
            show_aged_care_jobs: true,
            salary_range: (20.0, 120.0),
            time_filter: TimeFilter::All,
            sort_by: SortOption::Distance,
            location_radius: 50.0,
        }
    }
}

#[derive(Clone, Debug)]
struct LiveJobUpdate {
    job: Job,
    update_type: UpdateType,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug)]
enum UpdateType {
    New,
    Updated,
    Removed,
    Urgent,
}

// ============================================================================
// Comprehensive Map Component with Advanced Features
// ============================================================================

#[component]
pub fn MapPageComprehensive() -> Element {
    // ========================================================================
    // Core State Management - Comprehensive Feature Set
    // ========================================================================
    
    // Job data and filtering
    let mut jobs = use_signal(|| get_comprehensive_australian_jobs());
    let mut filtered_jobs = use_signal(|| Vec::<Job>::new());
    let mut selected_job = use_signal(|| None::<Job>);
    let mut saved_jobs = use_signal(|| Vec::<JobId>::new());
    let mut applied_jobs = use_signal(|| Vec::<JobId>::new());
    
    // Location management with advanced fallback system
    let mut user_location = use_signal(|| None::<LocationData>);
    let mut location_permission_state = use_signal(|| PermissionState::Unknown);
    let mut show_location_permission_dialog = use_signal(|| false);
    let mut location_loading = use_signal(|| false);
    let mut location_error = use_signal(|| None::<String>);
    
    // Advanced filtering system
    let mut filters = use_signal(|| JobFilters::default());
    let mut show_filters_panel = use_signal(|| false);
    let mut active_filters_count = use_signal(|| 0);
    let mut filter_presets = use_signal(|| Vec::<FilterPreset>::new());
    
    // Map visualization controls
    let mut map_style = use_signal(|| MapStyle::Light);
    let mut show_heatmap = use_signal(|| false);
    let mut heatmap_type = use_signal(|| HeatmapType::Density);
    let mut show_3d_buildings = use_signal(|| false);
    let mut show_traffic = use_signal(|| false);
    let mut show_clustering = use_signal(|| true);
    let mut zoom_level = use_signal(|| 11.0);
    let mut map_center = use_signal(|| (-34.9285, 138.6007)); // Adelaide default
    
    // Interactive features
    let mut show_live_feed = use_signal(|| false);
    let mut live_job_updates = use_signal(|| Vec::<LiveJobUpdate>::new());
    let mut show_commute_calculator = use_signal(|| false);
    let mut commute_info = use_signal(|| None::<CommuteInfo>);
    let mut show_directions = use_signal(|| false);
    let mut directions_route = use_signal(|| None::<Vec<(f64, f64)>>);
    let mut auto_refresh = use_signal(|| true);
    let mut refresh_interval = use_signal(|| 30); // seconds
    
    // UI state management
    let mut loading_state = use_signal(|| LoadingState::Initial);
    let mut error_boundary = use_signal(|| None::<String>);
    let mut toast_notifications = use_signal(|| Vec::<ToastNotification>::new());
    let mut theme_mode = use_signal(|| ThemeMode::Light);
    let mut sound_enabled = use_signal(|| true);
    let mut haptic_feedback = use_signal(|| true);
    
    // Performance monitoring
    let mut render_time = use_signal(|| 0.0);
    let mut api_response_time = use_signal(|| 0.0);
    let mut viewport_jobs = use_signal(|| Vec::<Job>::new());
    let mut virtual_scroll_offset = use_signal(|| 0);
    
    // Advanced features state
    let mut job_detail_panel = use_signal(|| None::<Job>);
    let mut sharing_job = use_signal(|| None::<Job>);
    let mut applying_to_job = use_signal(|| None::<JobId>);
    let mut search_history = use_signal(|| Vec::<String>::new());
    let mut recently_viewed = use_signal(|| Vec::<JobId>::new());
    
    // Statistics and analytics
    let total_jobs = filtered_jobs.read().len();
    let urgent_jobs = filtered_jobs.read().iter().filter(|j| j.is_urgent).count();
    let avg_rate = if total_jobs > 0 {
        filtered_jobs.read().iter().map(|j| j.hourly_rate).sum::<f64>() / total_jobs as f64
    } else { 0.0 };
    let nearby_jobs = if let Some(location) = user_location.read().as_ref() {
        filtered_jobs.read().iter()
            .filter(|job| {
                if let (Some(lat), Some(lng)) = (job.latitude, job.longitude) {
                    calculate_distance(location.coordinates, (lat, lng)) <= filters.read().location_radius
                } else { false }
            })
            .count()
    } else { 0 };

    // ========================================================================
    // Advanced Effects and Business Logic
    // ========================================================================

    // Comprehensive job filtering with performance optimization
    use_effect(move || {
        let start_time = web_sys::js_sys::Date::now();
        let filter_state = filters.read();
        let all_jobs = jobs.read();
        
        let mut filtered: Vec<Job> = all_jobs.iter()
            .filter(|job| apply_comprehensive_filters(job, &filter_state))
            .cloned()
            .collect();
        
        // Apply sorting with distance calculation if user location available
        if let Some(location) = user_location.read().as_ref() {
            sort_jobs_by_criteria(&mut filtered, &filter_state.sort_by, Some(location.coordinates));
        } else {
            sort_jobs_by_criteria(&mut filtered, &filter_state.sort_by, None);
        }
        
        filtered_jobs.set(filtered);
        
        // Update performance metrics
        let end_time = web_sys::js_sys::Date::now();
        render_time.set(end_time - start_time);
    });

    // Advanced location management with progressive fallback
    use_effect(move || {
        if *location_permission_state.read() == PermissionState::Unknown {
            spawn(async move {
                location_loading.set(true);
                location_error.set(None);
                
                // Try GPS first
                if let Some(gps_location) = try_get_gps_location().await {
                    user_location.set(Some(LocationData {
                        coordinates: gps_location,
                        accuracy: Some(25.0),
                        source: LocationSource::GPS,
                        timestamp: Utc::now(),
                        address: None,
                    }));
                    location_permission_state.set(PermissionState::Granted);
                }
                // Fall back to profile location
                else if let Some(profile_location) = get_profile_location().await {
                    user_location.set(Some(LocationData {
                        coordinates: profile_location,
                        accuracy: None,
                        source: LocationSource::Profile,
                        timestamp: Utc::now(),
                        address: None,
                    }));
                    location_permission_state.set(PermissionState::Profile);
                }
                // Fall back to cached location
                else if let Some(cached_location) = get_cached_location() {
                    user_location.set(Some(cached_location));
                    location_permission_state.set(PermissionState::Cached);
                }
                // Default to Adelaide
                else {
                    user_location.set(Some(LocationData {
                        coordinates: (-34.9285, 138.6007),
                        accuracy: None,
                        source: LocationSource::Default,
                        timestamp: Utc::now(),
                        address: Some("Adelaide, SA".to_string()),
                    }));
                    location_permission_state.set(PermissionState::Default);
                }
                
                location_loading.set(false);
            });
        }
    });

    // Live job feed simulation with WebSocket-style updates
    use_effect(move || {
        if *show_live_feed.read() && *auto_refresh.read() {
            spawn(async move {
                loop {
                    TimeoutFuture::new(*refresh_interval.read() as u32 * 1000).await;
                    
                    // Simulate new job arrival
                    if let Some(new_job) = simulate_live_job_update().await {
                        let update = LiveJobUpdate {
                            job: new_job.clone(),
                            update_type: UpdateType::New,
                            timestamp: Utc::now(),
                        };
                        
                        let mut updates = live_job_updates.read().clone();
                        updates.insert(0, update);
                        if updates.len() > 10 {
                            updates.truncate(10);
                        }
                        live_job_updates.set(updates);
                        
                        // Add to jobs list
                        let mut current_jobs = jobs.read().clone();
                        current_jobs.insert(0, new_job);
                        jobs.set(current_jobs);
                        
                        // Show notification
                        if *sound_enabled.read() {
                            play_notification_sound();
                        }
                        
                        show_toast_notification("New job available!", ToastType::Success);
                    }
                    
                    if !*auto_refresh.read() { break; }
                }
            });
        }
    });

    // URL state synchronization for filters
    use_effect(move || {
        if let Some(window) = window() {
            if let Ok(url) = window.location().href() {
                if let Ok(url_obj) = web_sys::Url::new(&url) {
                    let search_params = url_obj.search_params();
                    sync_filters_with_url(&search_params, &mut filters);
                }
            }
        }
    });

    // Performance monitoring and optimization
    use_effect(move || {
        // Monitor performance metrics
        spawn(async move {
            TimeoutFuture::new(1000).await; // Every second
            
            // Update viewport-based job loading
            let viewport_bounds = get_map_viewport_bounds().await;
            let viewport_filtered = filtered_jobs.read().iter()
                .filter(|job| is_job_in_viewport(job, &viewport_bounds))
                .cloned()
                .collect();
            viewport_jobs.set(viewport_filtered);
        });
    });

    // ========================================================================
    // Advanced Apple.com Style Glass Morphism UI
    // ========================================================================

    rsx! {
        div { 
            class: "h-screen w-full flex flex-col bg-gradient-to-br from-blue-50 via-indigo-50 to-purple-50 dark:from-slate-900 dark:via-blue-900 dark:to-indigo-900 overflow-hidden transition-all duration-500",
            
            // Advanced Header with Glass Morphism
            header { 
                class: "sticky top-0 z-50 bg-white/20 dark:bg-black/20 backdrop-blur-2xl border-b border-white/10 shadow-2xl",
                div { 
                    class: "max-w-8xl mx-auto px-6 py-4",
                    
                    // Top header row with logo and controls
                    div { class: "flex items-center justify-between mb-4",
                        // Sophisticated logo and branding
                        div { class: "flex items-center space-x-4",
                            div { 
                                class: "w-12 h-12 bg-gradient-to-br from-blue-500 via-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center shadow-xl transform hover:scale-105 transition-all duration-300",
                                span { class: "text-white font-bold text-xl", "🗺️" }
                            }
                            div { class: "flex flex-col",
                                h1 { class: "text-2xl font-bold bg-gradient-to-r from-blue-600 via-indigo-600 to-purple-600 bg-clip-text text-transparent",
                                    "Loco Connect Map"
                                }
                                p { class: "text-sm text-gray-500 dark:text-gray-400", "Find your perfect pharmacy role" }
                            }
                        }
                        
                        // Advanced header controls
                        div { class: "flex items-center space-x-3",
                            // Live status indicator
                            if *show_live_feed.read() {
                                div { class: "flex items-center space-x-2 px-3 py-2 bg-green-500/20 backdrop-blur-lg rounded-xl border border-green-500/30",
                                    div { class: "w-2 h-2 bg-green-500 rounded-full animate-pulse" }
                                    span { class: "text-xs font-medium text-green-700 dark:text-green-300", "Live" }
                                }
                            }
                            
                            // Location status with advanced styling
                            if let Some(location) = user_location.read().as_ref() {
                                div { class: "flex items-center space-x-2 px-3 py-2 bg-teal-500/20 backdrop-blur-lg rounded-xl border border-teal-500/30",
                                    span { class: "text-teal-600 dark:text-teal-400 text-sm", "📍" }
                                    span { class: "text-xs font-medium text-teal-700 dark:text-teal-300", 
                                        match location.source {
                                            LocationSource::GPS => "GPS Active",
                                            LocationSource::Profile => "Profile Location", 
                                            LocationSource::Cached => "Cached Location",
                                            LocationSource::Default => "Default Location",
                                        }
                                    }
                                    if let Some(accuracy) = location.accuracy {
                                        span { class: "text-xs text-teal-600 dark:text-teal-400", "±{accuracy:.0}m" }
                                    }
                                }
                            }
                            
                            // Theme toggle with smooth animation
                            button {
                                class: "glass-button-premium p-3 rounded-xl transition-all duration-300 hover:scale-105",
                                onclick: move |_| {
                                    let current = theme_mode.read().clone();
                                    theme_mode.set(match current {
                                        ThemeMode::Light => ThemeMode::Dark,
                                        ThemeMode::Dark => ThemeMode::Light,
                                    });
                                },
                                span { class: "text-xl", 
                                    match *theme_mode.read() {
                                        ThemeMode::Light => "🌙",
                                        ThemeMode::Dark => "☀️",
                                    }
                                }
                            }
                            
                            // Settings panel toggle
                            button {
                                class: if *show_filters_panel.read() { "glass-button-active p-3 rounded-xl" } else { "glass-button-premium p-3 rounded-xl" },
                                onclick: move |_| {
                                    let current = *show_filters_panel.read();
                                    show_filters_panel.set(!current);
                                },
                                span { class: "text-xl", "⚙️" }
                            }
                        }
                    }
                    
                    // Advanced search bar with intelligent features
                    div { class: "relative",
                        div { class: "relative group",
                            input {
                                r#type: "text",
                                placeholder: "Search jobs, locations, employers, or skills...",
                                class: "w-full px-6 py-4 pl-14 pr-20 bg-white/30 dark:bg-black/20 backdrop-blur-xl rounded-2xl border border-white/20 focus:border-blue-400/50 focus:ring-4 focus:ring-blue-400/10 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 transition-all duration-300 shadow-xl text-lg",
                                value: filters.read().search_query.clone(),
                                oninput: move |e| {
                                    let mut filter_state = filters.read().clone();
                                    filter_state.search_query = e.value();
                                    filters.set(filter_state);
                                    
                                    // Add to search history
                                    if !e.value().is_empty() {
                                        let mut history = search_history.read().clone();
                                        if !history.contains(&e.value()) {
                                            history.insert(0, e.value());
                                            if history.len() > 10 {
                                                history.truncate(10);
                                            }
                                            search_history.set(history);
                                        }
                                    }
                                }
                            }
                            
                            // Search icon with glass effect
                            div { class: "absolute left-4 top-1/2 transform -translate-y-1/2",
                                div { class: "w-6 h-6 bg-gradient-to-r from-blue-500 to-indigo-500 rounded-lg flex items-center justify-center",
                                    span { class: "text-white text-sm", "🔍" }
                                }
                            }
                            
                            // Advanced filter count badge
                            if *active_filters_count.read() > 0 {
                                div { 
                                    class: "absolute right-4 top-1/2 transform -translate-y-1/2 bg-gradient-to-r from-blue-500 to-indigo-500 text-white px-3 py-1 rounded-full text-sm font-semibold shadow-xl animate-pulse",
                                    "{active_filters_count.read()} filters"
                                }
                            }
                        }
                        
                        // Search suggestions dropdown (when search is active)
                        if !filters.read().search_query.is_empty() && !search_history.read().is_empty() {
                            div { class: "absolute top-full left-0 right-0 mt-2 bg-white/90 dark:bg-black/80 backdrop-blur-xl rounded-xl border border-white/20 shadow-2xl z-10",
                                div { class: "p-2",
                                    h3 { class: "text-sm font-medium text-gray-700 dark:text-gray-300 px-3 py-2", "Recent Searches" }
                                    for search_term in search_history.read().iter().take(5) {
                                        button {
                                            class: "w-full text-left px-3 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-blue-500/10 rounded-lg transition-all duration-200",
                                            onclick: move |_| {
                                                let mut filter_state = filters.read().clone();
                                                filter_state.search_query = search_term.clone();
                                                filters.set(filter_state);
                                            },
                                            "{search_term}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Quick filter chips
                    div { class: "flex items-center space-x-3 mt-4 overflow-x-auto",
                        // Urgent jobs filter
                        button {
                            class: if filters.read().show_urgent_only { "filter-chip-active" } else { "filter-chip" },
                            onclick: move |_| {
                                let mut filter_state = filters.read().clone();
                                filter_state.show_urgent_only = !filter_state.show_urgent_only;
                                filters.set(filter_state);
                            },
                            span { class: "mr-2", "🚨" }
                            "Urgent Only"
                            if filters.read().show_urgent_only {
                                span { class: "ml-2 bg-white/30 rounded-full px-2 py-1 text-xs", urgent_jobs.to_string() }
                            }
                        }
                        
                        // Location radius filter
                        button {
                            class: "filter-chip",
                            onclick: move |_| {
                                // Toggle radius between 10, 25, 50, 100km
                                let mut filter_state = filters.read().clone();
                                filter_state.location_radius = match filter_state.location_radius {
                                    10.0 => 25.0,
                                    25.0 => 50.0,
                                    50.0 => 100.0,
                                    _ => 10.0,
                                };
                                filters.set(filter_state);
                            },
                            span { class: "mr-2", "📍" }
                            "Within {filters.read().location_radius:.0}km"
                            if nearby_jobs > 0 {
                                span { class: "ml-2 bg-white/30 rounded-full px-2 py-1 text-xs", nearby_jobs.to_string() }
                            }
                        }
                        
                        // Salary range quick filter
                        button {
                            class: if filters.read().salary_range != (20.0, 120.0) { "filter-chip-active" } else { "filter-chip" },
                            onclick: move |_| {
                                // Cycle through common salary ranges
                                let mut filter_state = filters.read().clone();
                                filter_state.salary_range = match filter_state.salary_range {
                                    (20.0, 120.0) => (40.0, 80.0),
                                    (40.0, 80.0) => (60.0, 100.0),
                                    (60.0, 100.0) => (80.0, 120.0),
                                    _ => (20.0, 120.0),
                                };
                                filters.set(filter_state);
                            },
                            span { class: "mr-2", "💰" }
                            "${filters.read().salary_range.0:.0}-${filters.read().salary_range.1:.0}/hr"
                        }
                        
                        // Job type filter
                        if !filters.read().show_hospital_jobs || !filters.read().show_retail_jobs || !filters.read().show_clinical_jobs {
                            button {
                                class: "filter-chip-active",
                                onclick: move |_| {
                                    let mut filter_state = filters.read().clone();
                                    filter_state.show_hospital_jobs = true;
                                    filter_state.show_retail_jobs = true;
                                    filter_state.show_clinical_jobs = true;
                                    filter_state.show_locum_jobs = true;
                                    filter_state.show_compounding_jobs = true;
                                    filter_state.show_aged_care_jobs = true;
                                    filters.set(filter_state);
                                },
                                span { class: "mr-2", "🏥" }
                                "Job Types"
                                span { class: "ml-2", "✕" }
                            }
                        }
                    }
                }
            }
            
            // Main content area with advanced layout
            div { class: "flex-1 flex overflow-hidden",
                
                // Advanced filters panel (collapsible with smooth animation)
                if *show_filters_panel.read() {
                    aside { 
                        class: "w-96 bg-white/10 dark:bg-black/10 backdrop-blur-2xl border-r border-white/10 shadow-2xl overflow-y-auto transition-all duration-500 transform",
                        div { class: "p-6 space-y-8",
                            // Panel header with close button
                            div { class: "flex items-center justify-between mb-6",
                                h3 { class: "text-xl font-bold text-gray-900 dark:text-white flex items-center",
                                    span { class: "mr-3 text-2xl", "🎛️" }
                                    "Advanced Filters"
                                }
                                button {
                                    class: "glass-button-small p-2 rounded-lg",
                                    onclick: move |_| show_filters_panel.set(false),
                                    span { class: "text-lg", "✕" }
                                }
                            }
                            
                            // Professional type filters with beautiful styling
                            div { class: "filter-section",
                                h4 { class: "filter-section-title", "🏥 Professional Types" }
                                div { class: "grid grid-cols-2 gap-3",
                                    FilterToggleCard {
                                        icon: "🏥",
                                        label: "Hospital",
                                        active: filters.read().show_hospital_jobs,
                                        count: jobs.read().iter().filter(|j| j.job_type == JobType::Pharmacist && j.pharmacy_name.contains("Hospital")).count(),
                                        on_toggle: move |_| {
                                            let mut filter_state = filters.read().clone();
                                            filter_state.show_hospital_jobs = !filter_state.show_hospital_jobs;
                                            filters.set(filter_state);
                                        }
                                    }
                                    FilterToggleCard {
                                        icon: "🏬",
                                        label: "Retail",
                                        active: filters.read().show_retail_jobs,
                                        count: jobs.read().iter().filter(|j| j.job_type == JobType::Pharmacist && !j.pharmacy_name.contains("Hospital")).count(),
                                        on_toggle: move |_| {
                                            let mut filter_state = filters.read().clone();
                                            filter_state.show_retail_jobs = !filter_state.show_retail_jobs;
                                            filters.set(filter_state);
                                        }
                                    }
                                    FilterToggleCard {
                                        icon: "🔬",
                                        label: "Clinical",
                                        active: filters.read().show_clinical_jobs,
                                        count: jobs.read().iter().filter(|j| j.job_type == JobType::PharmacyTechnician).count(),
                                        on_toggle: move |_| {
                                            let mut filter_state = filters.read().clone();
                                            filter_state.show_clinical_jobs = !filter_state.show_clinical_jobs;
                                            filters.set(filter_state);
                                        }
                                    }
                                    FilterToggleCard {
                                        icon: "💼",
                                        label: "Locum",
                                        active: filters.read().show_locum_jobs,
                                        count: jobs.read().iter().filter(|j| j.job_type == JobType::Intern).count(),
                                        on_toggle: move |_| {
                                            let mut filter_state = filters.read().clone();
                                            filter_state.show_locum_jobs = !filter_state.show_locum_jobs;
                                            filters.set(filter_state);
                                        }
                                    }
                                }
                            }
                            
                            // Advanced salary range slider
                            div { class: "filter-section",
                                h4 { class: "filter-section-title", 
                                    "💰 Salary Range: ${filters.read().salary_range.0:.0} - ${filters.read().salary_range.1:.0}/hr"
                                }
                                div { class: "space-y-4",
                                    // Custom range slider would go here
                                    div { class: "flex space-x-2",
                                        button {
                                            class: "salary-preset-btn",
                                            onclick: move |_| {
                                                let mut filter_state = filters.read().clone();
                                                filter_state.salary_range = (20.0, 50.0);
                                                filters.set(filter_state);
                                            },
                                            "$20-50"
                                        }
                                        button {
                                            class: "salary-preset-btn",
                                            onclick: move |_| {
                                                let mut filter_state = filters.read().clone();
                                                filter_state.salary_range = (50.0, 80.0);
                                                filters.set(filter_state);
                                            },
                                            "$50-80"
                                        }
                                        button {
                                            class: "salary-preset-btn",
                                            onclick: move |_| {
                                                let mut filter_state = filters.read().clone();
                                                filter_state.salary_range = (80.0, 120.0);
                                                filters.set(filter_state);
                                            },
                                            "$80-120"
                                        }
                                    }
                                }
                            }
                            
                            // Time-based filtering
                            div { class: "filter-section",
                                h4 { class: "filter-section-title", "⏰ Shift Times" }
                                div { class: "grid grid-cols-1 gap-2",
                                    for time_option in [
                                        (TimeFilter::All, "All Times", "🕐"),
                                        (TimeFilter::Morning, "Morning (6AM-2PM)", "🌅"),
                                        (TimeFilter::Afternoon, "Afternoon (2PM-10PM)", "☀️"),
                                        (TimeFilter::Evening, "Evening (6PM-12AM)", "🌙"),
                                    ] {
                                        button {
                                            class: if filters.read().time_filter == time_option.0 { "time-filter-active" } else { "time-filter" },
                                            onclick: move |_| {
                                                let mut filter_state = filters.read().clone();
                                                filter_state.time_filter = time_option.0.clone();
                                                filters.set(filter_state);
                                            },
                                            span { class: "mr-3", time_option.2 }
                                            time_option.1
                                        }
                                    }
                                }
                            }
                            
                            // Sort options with beautiful styling
                            div { class: "filter-section",
                                h4 { class: "filter-section-title", "📊 Sort By" }
                                div { class: "grid grid-cols-1 gap-2",
                                    for sort_option in [
                                        (SortOption::Distance, "Distance", "📍"),
                                        (SortOption::Date, "Most Recent", "📅"),
                                        (SortOption::Rate, "Highest Pay", "💰"),
                                        (SortOption::Relevance, "Relevance", "⭐"),
                                    ] {
                                        button {
                                            class: if filters.read().sort_by == sort_option.0 { "sort-option-active" } else { "sort-option" },
                                            onclick: move |_| {
                                                let mut filter_state = filters.read().clone();
                                                filter_state.sort_by = sort_option.0.clone();
                                                filters.set(filter_state);
                                            },
                                            span { class: "mr-3", sort_option.2 }
                                            sort_option.1
                                        }
                                    }
                                }
                            }
                            
                            // Filter actions
                            div { class: "flex space-x-3 pt-6 border-t border-white/10",
                                button {
                                    class: "flex-1 bg-gradient-to-r from-blue-500 to-indigo-500 text-white px-4 py-3 rounded-xl font-semibold shadow-xl hover:shadow-2xl transform hover:scale-105 transition-all duration-300",
                                    onclick: move |_| {
                                        filters.set(JobFilters::default());
                                        show_toast_notification("Filters reset", ToastType::Info);
                                    },
                                    "Reset All"
                                }
                                button {
                                    class: "flex-1 bg-white/20 backdrop-blur-lg text-gray-900 dark:text-white px-4 py-3 rounded-xl font-semibold border border-white/30 hover:bg-white/30 transition-all duration-300",
                                    onclick: move |_| {
                                        // Save current filters as preset
                                        show_toast_notification("Filter preset saved", ToastType::Success);
                                    },
                                    "Save Preset"
                                }
                            }
                        }
                    }
                }
                
                // Main map container with advanced features
                main { 
                    class: "flex-1 relative bg-gradient-to-br from-blue-100 via-indigo-100 to-purple-100 dark:from-slate-800 dark:via-blue-800 dark:to-indigo-800",
                    
                    // Map area with sophisticated overlay
                    div { class: "absolute inset-0",
                        // Background pattern
                        div { class: "absolute inset-0 opacity-30",
                            style: "background-image: radial-gradient(circle at 25% 25%, rgba(59, 130, 246, 0.1) 0%, transparent 50%), radial-gradient(circle at 75% 75%, rgba(139, 92, 246, 0.1) 0%, transparent 50%)"
                        }
                        
                        // Map placeholder with glass overlay
                        div { class: "absolute inset-0 flex items-center justify-center",
                            div { class: "text-8xl opacity-20", "🗺️" }
                        }
                        
                        // Advanced map controls (top-right)
                        div { class: "absolute top-6 right-6 space-y-3",
                            // Map style selector
                            div { class: "glass-panel p-4 rounded-xl",
                                h4 { class: "text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3", "Map Style" }
                                div { class: "grid grid-cols-2 gap-2",
                                    for style in [MapStyle::Light, MapStyle::Streets, MapStyle::Satellite, MapStyle::Dark] {
                                        button {
                                            class: if *map_style.read() == style { "map-style-active" } else { "map-style" },
                                            onclick: move |_| map_style.set(style.clone()),
                                            style_name(&style)
                                        }
                                    }
                                }
                            }
                            
                            // Map layer controls
                            div { class: "glass-panel p-4 rounded-xl",
                                h4 { class: "text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3", "Layers" }
                                div { class: "space-y-2",
                                    ToggleControl {
                                        label: "🔥 Heatmap",
                                        active: *show_heatmap.read(),
                                        on_toggle: move |_| {
                                            let current = *show_heatmap.read();
                                            show_heatmap.set(!current);
                                        }
                                    }
                                    ToggleControl {
                                        label: "🏢 3D Buildings",
                                        active: *show_3d_buildings.read(),
                                        on_toggle: move |_| {
                                            let current = *show_3d_buildings.read();
                                            show_3d_buildings.set(!current);
                                        }
                                    }
                                    ToggleControl {
                                        label: "🚗 Traffic",
                                        active: *show_traffic.read(),
                                        on_toggle: move |_| {
                                            let current = *show_traffic.read();
                                            show_traffic.set(!current);
                                        }
                                    }
                                    ToggleControl {
                                        label: "📍 Clustering",
                                        active: *show_clustering.read(),
                                        on_toggle: move |_| {
                                            let current = *show_clustering.read();
                                            show_clustering.set(!current);
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Live job feed (bottom-left)
                        if *show_live_feed.read() {
                            div { class: "absolute bottom-6 left-6 w-80",
                                LiveJobFeedPanel {
                                    updates: live_job_updates.read().clone(),
                                    auto_refresh: *auto_refresh.read(),
                                    on_toggle_refresh: move |_| {
                                        let current = *auto_refresh.read();
                                        auto_refresh.set(!current);
                                    },
                                    on_job_click: move |job| {
                                        job_detail_panel.set(Some(job));
                                    }
                                }
                            }
                        }
                        
                        // Map statistics overlay (bottom-center)
                        div { class: "absolute bottom-6 left-1/2 transform -translate-x-1/2",
                            div { class: "glass-panel px-6 py-3 rounded-2xl",
                                div { class: "flex items-center space-x-6 text-sm",
                                    div { class: "flex items-center space-x-2",
                                        span { class: "text-blue-500", "📍" }
                                        span { class: "font-semibold text-gray-900 dark:text-white", "{total_jobs}" }
                                        span { class: "text-gray-600 dark:text-gray-400", "jobs" }
                                    }
                                    div { class: "flex items-center space-x-2",
                                        span { class: "text-red-500 animate-pulse", "🚨" }
                                        span { class: "font-semibold text-gray-900 dark:text-white", "{urgent_jobs}" }
                                        span { class: "text-gray-600 dark:text-gray-400", "urgent" }
                                    }
                                    div { class: "flex items-center space-x-2",
                                        span { class: "text-green-500", "💰" }
                                        span { class: "font-semibold text-gray-900 dark:text-white", "${avg_rate:.0}" }
                                        span { class: "text-gray-600 dark:text-gray-400", "avg/hr" }
                                    }
                                    if nearby_jobs > 0 {
                                        div { class: "flex items-center space-x-2",
                                            span { class: "text-teal-500", "📍" }
                                            span { class: "font-semibold text-gray-900 dark:text-white", "{nearby_jobs}" }
                                            span { class: "text-gray-600 dark:text-gray-400", "nearby" }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Performance indicator (top-left)
                        div { class: "absolute top-6 left-6",
                            div { class: "glass-panel px-3 py-2 rounded-lg",
                                div { class: "flex items-center space-x-2 text-xs",
                                    span { class: "text-yellow-500", "⚡" }
                                    span { class: "text-gray-700 dark:text-gray-300", "{render_time.read():.1}ms" }
                                    if *loading_state.read() == LoadingState::Loading {
                                        div { class: "w-2 h-2 bg-blue-500 rounded-full animate-pulse" }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Advanced job list panel (right side)
                aside { 
                    class: "w-96 bg-white/10 dark:bg-black/10 backdrop-blur-2xl border-l border-white/10 shadow-2xl overflow-hidden flex flex-col",
                    
                    // Job list header
                    div { class: "p-6 border-b border-white/10",
                        div { class: "flex items-center justify-between mb-4",
                            h3 { class: "text-lg font-bold text-gray-900 dark:text-white",
                                "Jobs ({total_jobs})"
                            }
                            div { class: "flex items-center space-x-2",
                                // List view toggle
                                button {
                                    class: "glass-button-small p-2 rounded-lg",
                                    onclick: move |_| {
                                        // Toggle between list and grid view
                                    },
                                    span { class: "text-sm", "⊞" }
                                }
                                // Sort indicator
                                span { class: "text-xs text-gray-500 dark:text-gray-400",
                                    match filters.read().sort_by {
                                        SortOption::Distance => "📍 Distance",
                                        SortOption::Date => "📅 Recent",
                                        SortOption::Rate => "💰 Pay",
                                        SortOption::Relevance => "⭐ Relevance",
                                    }
                                }
                            }
                        }
                        
                        // Quick actions
                        div { class: "flex space-x-2",
                            button {
                                class: "flex-1 glass-button-small px-3 py-2 rounded-lg text-sm",
                                onclick: move |_| {
                                    show_live_feed.set(!*show_live_feed.read());
                                },
                                if *show_live_feed.read() { "📡 Live On" } else { "📡 Live Off" }
                            }
                            button {
                                class: "flex-1 glass-button-small px-3 py-2 rounded-lg text-sm",
                                onclick: move |_| {
                                    show_commute_calculator.set(!*show_commute_calculator.read());
                                },
                                "🚗 Commute"
                            }
                        }
                    }
                    
                    // Virtual job list for performance
                    div { class: "flex-1 overflow-y-auto",
                        VirtualJobList {
                            jobs: filtered_jobs.read().clone(),
                            selected_job: selected_job.read().clone(),
                            user_location: user_location.read().as_ref().map(|l| l.coordinates),
                            saved_jobs: saved_jobs.read().clone(),
                            applied_jobs: applied_jobs.read().clone(),
                            on_job_select: move |job| {
                                selected_job.set(Some(job.clone()));
                                job_detail_panel.set(Some(job));
                                
                                // Add to recently viewed
                                let mut recent = recently_viewed.read().clone();
                                recent.retain(|&id| id != job.id);
                                recent.insert(0, job.id);
                                if recent.len() > 20 {
                                    recent.truncate(20);
                                }
                                recently_viewed.set(recent);
                            },
                            on_job_save: move |job_id| {
                                let mut saved = saved_jobs.read().clone();
                                if saved.contains(&job_id) {
                                    saved.retain(|&id| id != job_id);
                                    show_toast_notification("Job removed from saved", ToastType::Info);
                                } else {
                                    saved.push(job_id);
                                    show_toast_notification("Job saved!", ToastType::Success);
                                }
                                saved_jobs.set(saved);
                            },
                            on_job_apply: move |job_id| {
                                applying_to_job.set(Some(job_id));
                                show_toast_notification("Opening application...", ToastType::Info);
                            },
                            on_job_share: move |job| {
                                sharing_job.set(Some(job));
                            }
                        }
                    }
                }
            }
            
            // Advanced footer with comprehensive controls
            footer { 
                class: "bg-white/10 dark:bg-black/10 backdrop-blur-2xl border-t border-white/10 shadow-2xl",
                div { class: "max-w-8xl mx-auto px-6 py-4",
                    div { class: "flex items-center justify-between",
                        // Left side - Status indicators
                        div { class: "flex items-center space-x-6",
                            // Real-time job counter
                            div { class: "flex items-center space-x-2",
                                div { class: "w-3 h-3 bg-green-500 rounded-full animate-pulse" }
                                span { class: "text-sm font-medium text-gray-900 dark:text-white", 
                                    "Live: {total_jobs} jobs"
                                }
                                if !live_job_updates.read().is_empty() {
                                    span { class: "bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 px-2 py-1 rounded-full text-xs font-semibold",
                                        "+{live_job_updates.read().len()} updates"
                                    }
                                }
                            }
                            
                            // Location accuracy
                            if let Some(location) = user_location.read().as_ref() {
                                div { class: "flex items-center space-x-2 text-sm text-gray-600 dark:text-gray-400",
                                    span { class: "text-teal-500", "📍" }
                                    span { 
                                        match location.source {
                                            LocationSource::GPS => "GPS Active",
                                            LocationSource::Profile => "Profile",
                                            LocationSource::Cached => "Cached", 
                                            LocationSource::Default => "Default",
                                        }
                                    }
                                    if let Some(accuracy) = location.accuracy {
                                        span { class: "text-xs", "±{accuracy:.0}m" }
                                    }
                                }
                            }
                            
                            // Performance metrics
                            div { class: "flex items-center space-x-2 text-sm text-gray-500 dark:text-gray-400",
                                span { "⚡" }
                                span { "{render_time.read():.1}ms" }
                                if *api_response_time.read() > 0.0 {
                                    span { "📡 {api_response_time.read():.0}ms" }
                                }
                            }
                        }
                        
                        // Right side - Advanced controls
                        div { class: "flex items-center space-x-3",
                            // Live feed toggle
                            button {
                                class: if *show_live_feed.read() { "footer-control-active" } else { "footer-control" },
                                onclick: move |_| {
                                    let current = *show_live_feed.read();
                                    show_live_feed.set(!current);
                                    if current {
                                        show_toast_notification("Live feed disabled", ToastType::Info);
                                    } else {
                                        show_toast_notification("Live feed enabled", ToastType::Success);
                                    }
                                },
                                span { class: "mr-2", "📡" }
                                "Live Feed"
                            }
                            
                            // Auto-refresh control
                            button {
                                class: if *auto_refresh.read() { "footer-control-active" } else { "footer-control" },
                                onclick: move |_| {
                                    let current = *auto_refresh.read();
                                    auto_refresh.set(!current);
                                    show_toast_notification(
                                        if current { "Auto-refresh paused" } else { "Auto-refresh enabled" },
                                        ToastType::Info
                                    );
                                },
                                span { class: "mr-2", if *auto_refresh.read() { "⏸️" } else { "▶️" } }
                                if *auto_refresh.read() { "Auto-refresh" } else { "Paused" }
                                if *auto_refresh.read() {
                                    span { class: "ml-1 text-xs opacity-75", "({refresh_interval.read()}s)" }
                                }
                            }
                            
                            // Sound toggle
                            button {
                                class: if *sound_enabled.read() { "footer-control-active" } else { "footer-control" },
                                onclick: move |_| {
                                    let current = *sound_enabled.read();
                                    sound_enabled.set(!current);
                                },
                                span { class: "mr-2", if *sound_enabled.read() { "🔊" } else { "🔇" } }
                                "Sound"
                            }
                            
                            // Settings shortcut
                            button {
                                class: "footer-control",
                                onclick: move |_| {
                                    show_filters_panel.set(!*show_filters_panel.read());
                                },
                                span { class: "mr-2", "⚙️" }
                                "Settings"
                            }
                        }
                    }
                }
            }
            
            // Toast notification system
            ToastNotificationSystem {
                notifications: toast_notifications.read().clone(),
                on_dismiss: move |id| {
                    let mut notifications = toast_notifications.read().clone();
                    notifications.retain(|n| n.id != id);
                    toast_notifications.set(notifications);
                }
            }
            
            // Modal overlays
            if let Some(job) = job_detail_panel.read().as_ref() {
                JobDetailModal {
                    job: job.clone(),
                    user_location: user_location.read().as_ref().map(|l| l.coordinates),
                    is_saved: saved_jobs.read().contains(&job.id),
                    has_applied: applied_jobs.read().contains(&job.id),
                    on_close: move |_| job_detail_panel.set(None),
                    on_apply: move |job_id| {
                        applying_to_job.set(Some(job_id));
                        job_detail_panel.set(None);
                    },
                    on_save: move |job_id| {
                        let mut saved = saved_jobs.read().clone();
                        if saved.contains(&job_id) {
                            saved.retain(|&id| id != job_id);
                        } else {
                            saved.push(job_id);
                        }
                        saved_jobs.set(saved);
                    },
                    on_share: move |job| {
                        sharing_job.set(Some(job));
                    },
                    on_directions: move |job| {
                        show_directions.set(true);
                        selected_job.set(Some(job));
                    }
                }
            }
            
            // Location permission dialog
            if *show_location_permission_dialog.read() {
                LocationPermissionDialog {
                    on_allow: move |_| {
                        show_location_permission_dialog.set(false);
                        location_permission_state.set(PermissionState::Requesting);
                    },
                    on_deny: move |_| {
                        show_location_permission_dialog.set(false);
                        location_permission_state.set(PermissionState::Denied);
                    }
                }
            }
        }
    }
}

// ============================================================================
// Advanced Component Definitions
// ============================================================================

#[component]
fn FilterToggleCard(
    icon: String,
    label: String,
    active: bool,
    count: usize,
    on_toggle: EventHandler<()>
) -> Element {
    rsx! {
        button {
            class: if active { "filter-toggle-active" } else { "filter-toggle" },
            onclick: move |_| on_toggle.call(()),
            div { class: "flex items-center justify-between w-full",
                div { class: "flex items-center space-x-2",
                    span { class: "text-lg", "{icon}" }
                    span { class: "font-medium", "{label}" }
                }
                if count > 0 {
                    span { class: "filter-count", "{count}" }
                }
            }
        }
    }
}

#[component]
fn ToggleControl(
    label: String,
    active: bool,
    on_toggle: EventHandler<()>
) -> Element {
    rsx! {
        button {
            class: "w-full flex items-center justify-between p-3 rounded-lg transition-all duration-200 hover:bg-white/10",
            onclick: move |_| on_toggle.call(()),
            span { class: "text-sm font-medium text-gray-700 dark:text-gray-300", "{label}" }
            div { class: if active { "toggle-switch-active" } else { "toggle-switch" },
                div { class: "toggle-switch-thumb" }
            }
        }
    }
}

#[component]
fn LiveJobFeedPanel(
    updates: Vec<LiveJobUpdate>,
    auto_refresh: bool,
    on_toggle_refresh: EventHandler<()>,
    on_job_click: EventHandler<Job>
) -> Element {
    rsx! {
        div { class: "glass-panel p-4 rounded-xl",
            div { class: "flex items-center justify-between mb-4",
                h4 { class: "font-semibold text-gray-900 dark:text-white flex items-center",
                    span { class: "mr-2 text-red-500 animate-pulse", "🔴" }
                    "Live Job Feed"
                }
                button {
                    class: "glass-button-small p-2 rounded-lg",
                    onclick: move |_| on_toggle_refresh.call(()),
                    if auto_refresh { "⏸️" } else { "▶️" }
                }
            }
            div { class: "space-y-2 max-h-60 overflow-y-auto",
                if updates.is_empty() {
                    div { class: "text-center py-6 text-gray-500 dark:text-gray-400",
                        p { class: "text-sm", "No recent updates" }
                        p { class: "text-xs", "New jobs will appear here" }
                    }
                } else {
                    for update in updates.iter() {
                        LiveJobUpdateCard {
                            update: update.clone(),
                            on_click: move |job| on_job_click.call(job)
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LiveJobUpdateCard(
    update: LiveJobUpdate,
    on_click: EventHandler<Job>
) -> Element {
    let update_icon = match update.update_type {
        UpdateType::New => "🆕",
        UpdateType::Updated => "📝",
        UpdateType::Removed => "❌",
        UpdateType::Urgent => "🚨",
    };
    
    let time_ago = format_time_ago(update.timestamp);
    
    rsx! {
        button {
            class: "w-full p-3 bg-white/20 hover:bg-white/30 dark:bg-black/20 dark:hover:bg-black/30 rounded-lg transition-all duration-200 text-left border border-white/10",
            onclick: move |_| on_click.call(update.job.clone()),
            div { class: "flex items-start space-x-3",
                span { class: "text-lg flex-shrink-0", "{update_icon}" }
                div { class: "flex-1 min-w-0",
                    h5 { class: "font-medium text-gray-900 dark:text-white text-sm truncate",
                        "{update.job.title}"
                    }
                    p { class: "text-xs text-gray-600 dark:text-gray-400 truncate",
                        "{update.job.pharmacy_name}"
                    }
                    div { class: "flex items-center justify-between mt-1",
                        span { class: "text-xs font-semibold text-green-600 dark:text-green-400",
                            "${update.job.hourly_rate:.0}/hr"
                        }
                        span { class: "text-xs text-gray-500 dark:text-gray-400",
                            "{time_ago}"
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================
// Helper Functions and Utilities
// ============================================================================

fn apply_comprehensive_filters(job: &Job, filters: &JobFilters) -> bool {
    // Search query filter
    if !filters.search_query.is_empty() {
        let query = filters.search_query.to_lowercase();
        let searchable_text = format!(
            "{} {} {} {} {}",
            job.title.to_lowercase(),
            job.description.to_lowercase(),
            job.pharmacy_name.to_lowercase(),
            job.suburb.to_lowercase(),
            job.address.to_lowercase()
        );
        if !searchable_text.contains(&query) {
            return false;
        }
    }
    
    // Urgent filter
    if filters.show_urgent_only && !job.is_urgent {
        return false;
    }
    
    // Job type filters
    match job.job_type {
        JobType::Pharmacist => {
            if job.pharmacy_name.contains("Hospital") && !filters.show_hospital_jobs {
                return false;
            }
            if !job.pharmacy_name.contains("Hospital") && !filters.show_retail_jobs {
                return false;
            }
        },
        JobType::PharmacyTechnician => {
            if !filters.show_clinical_jobs {
                return false;
            }
        },
        JobType::Intern => {
            if !filters.show_locum_jobs {
                return false;
            }
        },
        JobType::PharmacyAssistant => {
            if !filters.show_compounding_jobs {
                return false;
            }
        },
    }
    
    // Salary range filter
    if job.hourly_rate < filters.salary_range.0 || job.hourly_rate > filters.salary_range.1 {
        return false;
    }
    
    // Time filter (simplified - in real app would check shift times)
    match filters.time_filter {
        TimeFilter::All => {},
        TimeFilter::Morning => {
            if !job.start_time.starts_with("0") && !job.start_time.starts_with("1") {
                return false;
            }
        },
        TimeFilter::Afternoon => {
            if !job.start_time.starts_with("1") && !job.start_time.starts_with("2") {
                return false;
            }
        },
        TimeFilter::Evening => {
            if !job.start_time.starts_with("1") && !job.start_time.starts_with("2") {
                return false;
            }
        },
    }
    
    true
}

fn sort_jobs_by_criteria(jobs: &mut Vec<Job>, sort_by: &SortOption, user_location: Option<(f64, f64)>) {
    match sort_by {
        SortOption::Distance => {
            if let Some(location) = user_location {
                jobs.sort_by(|a, b| {
                    let dist_a = if let (Some(lat), Some(lng)) = (a.latitude, a.longitude) {
                        calculate_distance(location, (lat, lng))
                    } else { f64::INFINITY };
                    let dist_b = if let (Some(lat), Some(lng)) = (b.latitude, b.longitude) {
                        calculate_distance(location, (lat, lng))
                    } else { f64::INFINITY };
                    dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        },
        SortOption::Date => {
            jobs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        },
        SortOption::Rate => {
            jobs.sort_by(|a, b| b.hourly_rate.partial_cmp(&a.hourly_rate).unwrap_or(std::cmp::Ordering::Equal));
        },
        SortOption::Relevance => {
            // Could implement relevance scoring based on user profile, search history, etc.
            jobs.sort_by(|a, b| {
                let score_a = calculate_relevance_score(a);
                let score_b = calculate_relevance_score(b);
                score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
            });
        },
    }
}

fn calculate_relevance_score(job: &Job) -> f64 {
    let mut score = 0.0;
    
    // Boost urgent jobs
    if job.is_urgent {
        score += 20.0;
    }
    
    // Boost higher paying jobs
    score += job.hourly_rate / 10.0;
    
    // Boost recent jobs
    let hours_since_posted = (Utc::now() - job.created_at).num_hours() as f64;
    score += (168.0 - hours_since_posted).max(0.0) / 10.0; // Max boost for jobs less than a week old
    
    // Could add more factors: user preferences, application history, etc.
    
    score
}

// Distance calculation using Haversine formula
fn calculate_distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    let (lat1, lng1) = point1;
    let (lat2, lng2) = point2;
    
    let r = 6371.0; // Earth's radius in kilometres
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();
    
    let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin() +
        lat1_rad.cos() * lat2_rad.cos() *
        (delta_lng / 2.0).sin() * (delta_lng / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    r * c
}

// Mock implementations for location services
async fn try_get_gps_location() -> Option<(f64, f64)> {
    // In real implementation, would use web_sys::Geolocation
    TimeoutFuture::new(1000).await;
    Some((-34.9285, 138.6007)) // Adelaide coordinates
}

async fn get_profile_location() -> Option<(f64, f64)> {
    // Would fetch from user profile
    None
}

fn get_cached_location() -> Option<LocationData> {
    // Would read from localStorage
    None
}

async fn simulate_live_job_update() -> Option<Job> {
    // Simulate random job updates
    if js_sys::Math::random() > 0.7 {
        let jobs = get_comprehensive_australian_jobs();
        if let Some(job) = jobs.first() {
            Some(job.clone())
        } else {
            None
        }
    } else {
        None
    }
}

fn play_notification_sound() {
    // Would play notification sound
    console::log_1(&"🔔 Notification sound".into());
}

fn show_toast_notification(message: &str, toast_type: ToastType) {
    console::log_1(&format!("🍞 {}: {}", 
        match toast_type {
            ToastType::Success => "Success",
            ToastType::Error => "Error", 
            ToastType::Info => "Info",
        },
        message
    ).into());
}

fn sync_filters_with_url(search_params: &UrlSearchParams, filters: &mut Signal<JobFilters>) {
    // Would sync filter state with URL parameters for bookmarking/sharing
}

async fn get_map_viewport_bounds() -> ViewportBounds {
    // Would get current map viewport bounds
    ViewportBounds {
        north: -34.0,
        south: -35.0,
        east: 139.0,
        west: 138.0,
    }
}

fn is_job_in_viewport(job: &Job, bounds: &ViewportBounds) -> bool {
    if let (Some(lat), Some(lng)) = (job.latitude, job.longitude) {
        lat >= bounds.south && lat <= bounds.north && lng >= bounds.west && lng <= bounds.east
    } else {
        false
    }
}

fn style_name(style: &MapStyle) -> &'static str {
    match style {
        MapStyle::Light => "Light",
        MapStyle::Streets => "Streets",
        MapStyle::Satellite => "Satellite", 
        MapStyle::Dark => "Dark",
    }
}

fn format_time_ago(timestamp: chrono::DateTime<chrono::Utc>) -> String {
    let duration = Utc::now() - timestamp;
    
    if duration.num_minutes() < 1 {
        "Just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else {
        format!("{}d ago", duration.num_days())
    }
}

// ============================================================================
// Enhanced Australian Job Data
// ============================================================================

fn get_comprehensive_australian_jobs() -> Vec<Job> {
    use uuid::Uuid;
    use chrono::{Utc, Duration};
    
    vec![
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Senior Clinical Pharmacist - ICU".to_string(),
            description: "Lead clinical pharmacy services in our state-of-the-art ICU. Collaborate with multidisciplinary teams to optimize medication therapy for critically ill patients. Provide clinical leadership and mentorship to junior staff.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 95.0,
            pharmacy_name: "Royal Adelaide Hospital".to_string(),
            address: "Port Road, Adelaide SA 5000".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9205),
            longitude: Some(138.6052),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(365),
            start_time: "07:00".to_string(),
            end_time: "15:30".to_string(),
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: None,
            created_at: Utc::now() - Duration::minutes(15),
            updated_at: Utc::now() - Duration::minutes(15),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Community Pharmacist - Burnside Village".to_string(),
            description: "Join our award-winning community pharmacy team in Adelaide's premium shopping precinct. Provide exceptional patient care, conduct medication reviews, and lead our vaccination program.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 78.0,
            pharmacy_name: "Burnside Village Pharmacy".to_string(),
            address: "447 Portrush Road, Burnside SA 5066".to_string(),
            suburb: "Burnside".to_string(),
            postcode: Postcode("5066".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9396),
            longitude: Some(138.6403),
            start_date: Utc::now() + Duration::days(14),
            end_date: Utc::now() + Duration::days(180),
            start_time: "08:30".to_string(),
            end_time: "17:00".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now() - Duration::hours(2),
            updated_at: Utc::now() - Duration::hours(2),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Locum Pharmacist - Multiple Sites".to_string(),
            description: "Flexible locum opportunities across Adelaide's northern suburbs. Perfect for experienced pharmacists seeking variety and excellent hourly rates. Choose your own schedule.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 82.0,
            pharmacy_name: "Adelaide Locum Network".to_string(),
            address: "Various locations, Adelaide SA".to_string(),
            suburb: "North Adelaide".to_string(),
            postcode: Postcode("5006".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9078),
            longitude: Some(138.5954),
            start_date: Utc::now() + Duration::days(3),
            end_date: Utc::now() + Duration::days(90),
            start_time: "09:00".to_string(),
            end_time: "17:30".to_string(),
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: None,
            created_at: Utc::now() - Duration::hours(4),
            updated_at: Utc::now() - Duration::hours(4),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Pharmacy Technician - Compounding".to_string(),
            description: "Specialized compounding technician role at Adelaide's premier compounding facility. Work with sterile preparations, chemotherapy, and complex formulations.".to_string(),
            job_type: JobType::PharmacyTechnician,
            hourly_rate: 52.0,
            pharmacy_name: "Adelaide Compounding Centre".to_string(),
            address: "123 Jetty Road, Glenelg SA 5045".to_string(),
            suburb: "Glenelg".to_string(),
            postcode: Postcode("5045".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9802),
            longitude: Some(138.5150),
            start_date: Utc::now() + Duration::days(21),
            end_date: Utc::now() + Duration::days(365),
            start_time: "08:00".to_string(),
            end_time: "16:30".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now() - Duration::hours(6),
            updated_at: Utc::now() - Duration::hours(6),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Retail Pharmacist - CBD Melbourne".to_string(),
            description: "Prime Collins Street location in Melbourne's heart. High-volume dispensary with excellent growth opportunities. Join our dynamic team serving the business district.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 85.0,
            pharmacy_name: "Collins Street Pharmacy".to_string(),
            address: "200 Collins Street, Melbourne VIC 3000".to_string(),
            suburb: "Melbourne".to_string(),
            postcode: Postcode("3000".to_string()),
            state: AustralianState::Victoria,
            latitude: Some(-37.8136),
            longitude: Some(144.9631),
            start_date: Utc::now() + Duration::days(10),
            end_date: Utc::now() + Duration::days(365),
            start_time: "07:30".to_string(),
            end_time: "18:00".to_string(),
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: None,
            created_at: Utc::now() - Duration::minutes(45),
            updated_at: Utc::now() - Duration::minutes(45),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Clinical Pharmacist - Emergency Department".to_string(),
            description: "Fast-paced ED role at Sydney's leading hospital. Provide immediate pharmaceutical care, drug information, and support to emergency medical teams.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 92.0,
            pharmacy_name: "Royal Prince Alfred Hospital".to_string(),
            address: "50 Missenden Road, Camperdown NSW 2050".to_string(),
            suburb: "Camperdown".to_string(),
            postcode: Postcode("2050".to_string()),
            state: AustralianState::NewSouthWales,
            latitude: Some(-33.8888),
            longitude: Some(151.1873),
            start_date: Utc::now() + Duration::days(28),
            end_date: Utc::now() + Duration::days(365),
            start_time: "06:00".to_string(),
            end_time: "14:30".to_string(),
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: None,
            created_at: Utc::now() - Duration::minutes(30),
            updated_at: Utc::now() - Duration::minutes(30),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Intern Pharmacist Program".to_string(),
            description: "Comprehensive intern program with rotations through hospital, community, and clinical settings. Excellent mentorship and clear pathway to registration.".to_string(),
            job_type: JobType::Intern,
            hourly_rate: 38.0,
            pharmacy_name: "SA Health Intern Network".to_string(),
            address: "11 Hindmarsh Square, Adelaide SA 5000".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9285),
            longitude: Some(138.6007),
            start_date: Utc::now() + Duration::days(60),
            end_date: Utc::now() + Duration::days(365),
            start_time: "08:00".to_string(),
            end_time: "16:30".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now() - Duration::days(1),
            updated_at: Utc::now() - Duration::days(1),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Pharmacy Assistant - Part Time".to_string(),
            description: "Support our busy community pharmacy team. Customer service focus with dispensary support. Great for students or those seeking flexible hours.".to_string(),
            job_type: JobType::PharmacyAssistant,
            hourly_rate: 32.0,
            pharmacy_name: "Prospect Road Pharmacy".to_string(),
            address: "89 Prospect Road, Prospect SA 5082".to_string(),
            suburb: "Prospect".to_string(),
            postcode: Postcode("5082".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.8851),
            longitude: Some(138.5947),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(180),
            start_time: "12:00".to_string(),
            end_time: "20:00".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now() - Duration::hours(8),
            updated_at: Utc::now() - Duration::hours(8),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Rural Pharmacist - Mount Gambier".to_string(),
            description: "Rewarding rural opportunity serving the Mount Gambier community. Competitive package including housing assistance and professional development support.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 88.0,
            pharmacy_name: "Mount Gambier Central Pharmacy".to_string(),
            address: "12 Commercial Street, Mount Gambier SA 5290".to_string(),
            suburb: "Mount Gambier".to_string(),
            postcode: Postcode("5290".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-37.8284),
            longitude: Some(140.7831),
            start_date: Utc::now() + Duration::days(45),
            end_date: Utc::now() + Duration::days(730),
            start_time: "08:30".to_string(),
            end_time: "17:30".to_string(),
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: None,
            created_at: Utc::now() - Duration::hours(3),
            updated_at: Utc::now() - Duration::hours(3),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Hospital Pharmacist - Weekend Shifts".to_string(),
            description: "Weekend specialist role at Flinders Medical Centre. Provide pharmaceutical care during weekends with excellent work-life balance and premium rates.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 98.0,
            pharmacy_name: "Flinders Medical Centre".to_string(),
            address: "Flinders Drive, Bedford Park SA 5042".to_string(),
            suburb: "Bedford Park".to_string(),
            postcode: Postcode("5042".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-35.0028),
            longitude: Some(138.5722),
            start_date: Utc::now() + Duration::days(14),
            end_date: Utc::now() + Duration::days(365),
            start_time: "08:00".to_string(),
            end_time: "16:00".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now() - Duration::hours(12),
            updated_at: Utc::now() - Duration::hours(12),
            created_by: UserId(Uuid::new_v4()),
        },
    ]
}

// ============================================================================
// Additional Type Definitions
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
enum PermissionState {
    Unknown,
    Requesting,
    Granted,
    Denied,
    Profile,
    Cached,
    Default,
}

#[derive(Clone, Debug, PartialEq)]
enum LoadingState {
    Initial,
    Loading,
    Success,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
enum ThemeMode {
    Light,
    Dark,
}

#[derive(Clone, Debug)]
struct ToastNotification {
    id: String,
    message: String,
    toast_type: ToastType,
    timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug)]
enum ToastType {
    Success,
    Error,
    Info,
}

#[derive(Clone, Debug)]
struct FilterPreset {
    name: String,
    filters: JobFilters,
}

#[derive(Clone, Debug)]
struct ViewportBounds {
    north: f64,
    south: f64,
    east: f64,
    west: f64,
}

// Placeholder component definitions that would be implemented
#[component]
fn VirtualJobList(
    jobs: Vec<Job>,
    selected_job: Option<Job>,
    user_location: Option<(f64, f64)>,
    saved_jobs: Vec<JobId>,
    applied_jobs: Vec<JobId>,
    on_job_select: EventHandler<Job>,
    on_job_save: EventHandler<JobId>,
    on_job_apply: EventHandler<JobId>,
    on_job_share: EventHandler<Job>
) -> Element {
    rsx! {
        div { class: "space-y-2 p-4",
            for job in jobs.iter().take(20) { // Virtual scrolling simulation
                VirtualJobCard {
                    job: job.clone(),
                    is_selected: selected_job.as_ref().map(|j| j.id) == Some(job.id),
                    user_location: user_location,
                    is_saved: saved_jobs.contains(&job.id),
                    has_applied: applied_jobs.contains(&job.id),
                    on_select: move |job| on_job_select.call(job),
                    on_save: move |job_id| on_job_save.call(job_id),
                    on_apply: move |job_id| on_job_apply.call(job_id),
                    on_share: move |job| on_job_share.call(job)
                }
            }
        }
    }
}

#[component]
fn VirtualJobCard(
    job: Job,
    is_selected: bool,
    user_location: Option<(f64, f64)>,
    is_saved: bool,
    has_applied: bool,
    on_select: EventHandler<Job>,
    on_save: EventHandler<JobId>,
    on_apply: EventHandler<JobId>,
    on_share: EventHandler<Job>
) -> Element {
    let distance = if let Some(user_loc) = user_location {
        if let (Some(lat), Some(lng)) = (job.latitude, job.longitude) {
            Some(calculate_distance(user_loc, (lat, lng)))
        } else { None }
    } else { None };
    
    rsx! {
        div { 
            class: if is_selected { "job-card-selected" } else { "job-card" },
            onclick: move |_| on_select.call(job.clone()),
            
            // Card header
            div { class: "flex items-start justify-between mb-3",
                div { class: "flex-1",
                    h3 { class: "font-semibold text-gray-900 dark:text-white text-sm mb-1 line-clamp-2",
                        "{job.title}"
                    }
                    p { class: "text-xs text-gray-600 dark:text-gray-400 flex items-center",
                        span { class: "mr-2", get_job_type_icon(&job.job_type) }
                        "{job.pharmacy_name}"
                    }
                }
                div { class: "text-right",
                    p { class: "font-bold text-green-600 dark:text-green-400 text-sm",
                        "${job.hourly_rate:.0}/hr"
                    }
                    if job.is_urgent {
                        span { class: "urgent-badge", "🚨 URGENT" }
                    }
                }
            }
            
            // Location and distance
            div { class: "flex items-center justify-between mb-3 text-xs text-gray-500 dark:text-gray-400",
                div { class: "flex items-center",
                    span { class: "mr-1", "📍" }
                    "{job.suburb}, {job.state:?}"
                }
                if let Some(dist) = distance {
                    span { class: "font-medium",
                        "{format_distance_km(dist)} away"
                    }
                }
            }
            
            // Quick actions
            div { class: "flex items-center justify-between",
                div { class: "flex space-x-2",
                    button {
                        class: if is_saved { "action-button-active" } else { "action-button" },
                        onclick: move |e| {
                            e.stop_propagation();
                            on_save.call(job.id);
                        },
                        span { class: "text-xs", if is_saved { "❤️" } else { "🤍" } }
                    }
                    button {
                        class: "action-button",
                        onclick: move |e| {
                            e.stop_propagation();
                            on_share.call(job.clone());
                        },
                        span { class: "text-xs", "📤" }
                    }
                }
                button {
                    class: if has_applied { "apply-button-applied" } else { "apply-button" },
                    onclick: move |e| {
                        e.stop_propagation();
                        on_apply.call(job.id);
                    },
                    if has_applied { "Applied ✓" } else { "Apply" }
                }
            }
        }
    }
}

fn get_job_type_icon(job_type: &JobType) -> &'static str {
    match job_type {
        JobType::Pharmacist => "🏥",
        JobType::PharmacyTechnician => "🔬",
        JobType::Intern => "💼",
        JobType::PharmacyAssistant => "🏬",
    }
}

fn format_distance_km(distance: f64) -> String {
    if distance < 1.0 {
        format!("{:.0}m", distance * 1000.0)
    } else {
        format!("{:.1}km", distance)
    }
}

// Placeholder for complex components that would be implemented
#[component]
fn JobDetailModal(
    job: Job,
    user_location: Option<(f64, f64)>,
    is_saved: bool,
    has_applied: bool,
    on_close: EventHandler<()>,
    on_apply: EventHandler<JobId>,
    on_save: EventHandler<JobId>,
    on_share: EventHandler<Job>,
    on_directions: EventHandler<Job>
) -> Element {
    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content-large",
                "Job Detail Modal - {job.title}"
                button { onclick: move |_| on_close.call(()), "Close" }
            }
        }
    }
}

#[component]
fn LocationPermissionDialog(
    on_allow: EventHandler<()>,
    on_deny: EventHandler<()>
) -> Element {
    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content",
                "Location Permission Dialog"
                button { onclick: move |_| on_allow.call(()), "Allow" }
                button { onclick: move |_| on_deny.call(()), "Deny" }
            }
        }
    }
}

#[component]
fn ToastNotificationSystem(
    notifications: Vec<ToastNotification>,
    on_dismiss: EventHandler<String>
) -> Element {
    rsx! {
        div { class: "toast-container",
            for notification in notifications.iter() {
                div { class: "toast",
                    "{notification.message}"
                    button { onclick: move |_| on_dismiss.call(notification.id.clone()), "×" }
                }
            }
        }
    }
}