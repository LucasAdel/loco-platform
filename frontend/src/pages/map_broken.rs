use dioxus::prelude::*;
use shared::types::{Job, JobType, JobId, UserId, Postcode, AustralianState, JobStatus, JobFilters};
use crate::components::job_card::JobCard;
use chrono::Utc;
use std::collections::HashMap;
use web_sys::{console, window};
use wasm_bindgen::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;

// Advanced data structures for enhanced features
#[derive(Clone, Debug)]
struct CommuteInfo {
    travel_time: f64,
    distance: f64,
    mode: String, // walking, driving, transit
    cost: Option<f64>,
}

#[derive(Clone, Debug)]
struct RouteInfo {
    start: (f64, f64),
    end: (f64, f64),
    waypoints: Vec<(f64, f64)>,
    duration: f64,
    instructions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
struct HeatmapPoint {
    lat: f64,
    lng: f64,
    intensity: f64,
    data_type: String, // salary, density, time
}

// Map page with comprehensive features
#[component]
pub fn MapPage() -> Element {
    // Core state management
    let mut jobs = use_signal(|| get_enhanced_mock_jobs());
    let mut filtered_jobs = use_signal(|| Vec::<Job>::new());
    let mut selected_job = use_signal(|| None::<Job>);
    let mut search_query = use_signal(|| String::new());
    let mut user_location = use_signal(|| None::<(f64, f64)>);
    let mut location_enabled = use_signal(|| false);
    let mut show_location_dialog = use_signal(|| false);
    
    // Filter state
    let mut show_urgent_only = use_signal(|| false);
    let mut show_hospital_jobs = use_signal(|| true);
    let mut show_retail_jobs = use_signal(|| true);
    let mut show_clinical_jobs = use_signal(|| true);
    let mut show_locum_jobs = use_signal(|| true);
    let mut show_compounding_jobs = use_signal(|| true);
    let mut show_aged_care_jobs = use_signal(|| true);
    let mut salary_range = use_signal(|| (20.0, 100.0));
    let mut time_filter = use_signal(|| "all".to_string()); // morning, afternoon, evening, all
    let mut sort_by = use_signal(|| "distance".to_string()); // distance, date, rate
    
    // Advanced filter state
    let mut show_filters_panel = use_signal(|| false);
    let mut active_filters_count = use_signal(|| 0);
    
    // Map visualization state
    let mut map_style = use_signal(|| "light".to_string()); // light, streets, satellite, dark
    let mut show_heatmap = use_signal(|| false);
    let mut heatmap_type = use_signal(|| "density".to_string()); // density, salary, time
    let mut show_3d_buildings = use_signal(|| false);
    let mut show_traffic = use_signal(|| false);
    let mut show_clustering = use_signal(|| true);
    
    // Interactive features state
    let mut show_live_feed = use_signal(|| false);
    let mut show_commute_calculator = use_signal(|| false);
    let mut show_directions = use_signal(|| false);
    let mut auto_refresh = use_signal(|| true);
    let mut sound_enabled = use_signal(|| true);
    let mut dark_mode = use_signal(|| false);
    
    // Advanced UI state
    let mut viewport_loading = use_signal(|| false);
    let mut virtual_scrolling = use_signal(|| true);
    let mut toast_message = use_signal(|| None::<String>);
    let mut error_message = use_signal(|| None::<String>);
    let mut loading_state = use_signal(|| false);
    
    // Performance and cache state
    let mut cache_enabled = use_signal(|| true);
    let mut last_refresh = use_signal(|| Utc::now());
    let mut api_request_count = use_signal(|| 0);
    
    // Location management state
    let mut location_source = use_signal(|| "unknown".to_string()); // gps, profile, cached, default
    let mut location_accuracy = use_signal(|| None::<f64>);
    let mut location_permission_requested = use_signal(|| false);
    
    // Advanced map controls
    let mut map_bounds = use_signal(|| None::<((f64, f64), (f64, f64))>);
    let mut zoom_level = use_signal(|| 11.0);
    let mut map_center = use_signal(|| (-34.9285, 138.6007)); // Adelaide default
    
    // Real-time and live features
    let mut live_job_count = use_signal(|| 0);
    let mut last_live_update = use_signal(|| Utc::now());
    let mut notification_count = use_signal(|| 0);
    
    // Advanced location features
    let mut location_history = use_signal(|| Vec::<(f64, f64)>::new());
    let mut nearest_jobs = use_signal(|| Vec::<Job>::new());
    
    // Commute and directions
    let mut commute_data = use_signal(|| None::<CommuteInfo>);
    let mut active_route = use_signal(|| None::<RouteInfo>);
    
    // Performance metrics
    let mut render_time = use_signal(|| 0.0);
    let mut api_response_time = use_signal(|| 0.0);
    
    // Performance state
    let mut viewport_loading = use_signal(|| false);
    let mut virtual_scrolling = use_signal(|| true);
    
    // Statistics
    let total_jobs = jobs.read().len();
    let urgent_jobs = jobs.read().iter().filter(|j| j.is_urgent).count();
    let avg_rate = if total_jobs > 0 {
        jobs.read().iter().map(|j| j.hourly_rate).sum::<f64>() / total_jobs as f64
    } else { 0.0 };

    // Filter jobs based on current criteria
    use_effect(move || {
        let query = search_query.read().to_lowercase();
        let jobs_list = jobs.read();
        
        let mut filtered: Vec<Job> = jobs_list.iter()
            .filter(|job| {
                // Search filter
                if !query.is_empty() {
                    let matches_search = job.title.to_lowercase().contains(&query) ||
                        job.description.to_lowercase().contains(&query) ||
                        job.pharmacy_name.to_lowercase().contains(&query) ||
                        job.suburb.to_lowercase().contains(&query);
                    if !matches_search { return false; }
                }
                
                // Urgent filter
                if *show_urgent_only.read() && !job.is_urgent {
                    return false;
                }
                
                // Job type filters
                match job.job_type {
                    JobType::Pharmacist if job.pharmacy_name.contains("Hospital") => *show_hospital_jobs.read(),
                    JobType::Pharmacist => *show_retail_jobs.read(),
                    JobType::PharmacyTechnician => *show_clinical_jobs.read(),
                    JobType::Intern => *show_locum_jobs.read(),
                    JobType::PharmacyAssistant => *show_compounding_jobs.read(),
                    _ => true
                }
            })
            .filter(|job| {
                // Salary range filter
                let range = salary_range.read();
                job.hourly_rate >= range.0 && job.hourly_rate <= range.1
            })
            .cloned()
            .collect();
        
        // Sort jobs
        match sort_by.read().as_str() {
            "distance" => {
                if let Some(location) = *user_location.read() {
                    filtered.sort_by(|a, b| {
                        let dist_a = calculate_distance(location, (a.latitude.unwrap_or(0.0), a.longitude.unwrap_or(0.0)));
                        let dist_b = calculate_distance(location, (b.latitude.unwrap_or(0.0), b.longitude.unwrap_or(0.0)));
                        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                    });
                }
            },
            "date" => filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at)),
            "rate" => filtered.sort_by(|a, b| b.hourly_rate.partial_cmp(&a.hourly_rate).unwrap_or(std::cmp::Ordering::Equal)),
            _ => {}
        }
        
        filtered_jobs.set(filtered);
    });

    // Calculate active filters count
    use_effect(move || {
        let mut count = 0;
        if !search_query.read().is_empty() { count += 1; }
        if *show_urgent_only.read() { count += 1; }
        if *salary_range.read() != (20.0, 100.0) { count += 1; }
        if time_filter.read().as_str() != "all" { count += 1; }
        if !*show_hospital_jobs.read() || !*show_retail_jobs.read() || 
           !*show_clinical_jobs.read() || !*show_locum_jobs.read() { count += 1; }
        
        active_filters_count.set(count);
    });

    // Advanced Location Effect - GPS with fallback strategy
    use_effect(move || {
        if *location_enabled.read() && !*location_permission_requested.read() {
            location_permission_requested.set(true);
            
            // Simulate GPS location acquisition with progressive fallback
            spawn(async move {
                // Step 1: Try GPS (simulate)
                user_location.set(Some((-34.9285, 138.6007)));
                location_source.set("gps".to_string());
                location_accuracy.set(Some(25.0));
                
                // Step 2: Update location history
                let mut history = location_history.read().clone();
                history.push((-34.9285, 138.6007));
                if history.len() > 10 { history.remove(0); }
                location_history.set(history);
            });
        }
    });

    // Live Feed Simulation Effect
    use_effect(move || {
        if *auto_refresh.read() {
            spawn(async move {
                // Simulate live job updates every 30 seconds
                TimeoutFuture::new(30_000).await;
                
                live_job_count.set(*live_job_count.read() + 1);
                last_live_update.set(Utc::now());
                
                // Simulate new job notification
                if *sound_enabled.read() {
                    console::log_1(&"🔔 New job notification".into());
                }
            });
        }
    });

    // Performance Monitoring Effect
    use_effect(move || {
        let start_time = web_sys::js_sys::Date::now();
        
        // Simulate render time calculation
        spawn(async move {
            TimeoutFuture::new(16).await; // 60fps target
            let end_time = web_sys::js_sys::Date::now();
            render_time.set(end_time - start_time);
        });
    });

    // Real-time Job Updates Effect (WebSocket simulation)
    use_effect(move || {
        if *show_live_feed.read() {
            spawn(async move {
                // Simulate WebSocket connection
                loop {
                    TimeoutFuture::new(5000).await;
                    
                    // Simulate receiving new job
                    notification_count.set(*notification_count.read() + 1);
                    
                    if !*auto_refresh.read() { break; }
                }
            });
        }
    });

    // Nearest Jobs Calculator Effect
    use_effect(move || {
        if let Some(user_loc) = *user_location.read() {
            let mut jobs_with_distance: Vec<_> = jobs.read().iter()
                .filter_map(|job| {
                    if let (Some(lat), Some(lng)) = (job.latitude, job.longitude) {
                        let distance = calculate_distance(user_loc, (lat, lng));
                        Some((job.clone(), distance))
                    } else {
                        None
                    }
                })
                .collect();
            
            jobs_with_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            
            let nearest: Vec<Job> = jobs_with_distance.into_iter()
                .take(5)
                .map(|(job, _)| job)
                .collect();
            
            nearest_jobs.set(nearest);
        }
    });

    rsx! {
        div { 
            class: "h-screen w-full flex flex-col bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 overflow-hidden",
            
            // Header with Search Bar and Controls
            header { 
                class: "sticky top-0 z-50 bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl border-b border-white/20 shadow-lg",
                div { 
                    class: "max-w-7xl mx-auto px-4 py-3 flex items-center justify-between",
                    
                    // Logo and Title
                    div { class: "flex items-center space-x-3",
                        div { 
                            class: "w-10 h-10 bg-gradient-to-r from-blue-500 to-indigo-600 rounded-xl flex items-center justify-center shadow-lg",
                            span { class: "text-white font-bold text-lg", "🗺️" }
                        }
                        h1 { class: "text-2xl font-bold bg-gradient-to-r from-blue-600 to-indigo-600 bg-clip-text text-transparent",
                            "Loco Connect Map"
                        }
                    }
                    
                    // Search Bar with Advanced Features
                    div { class: "flex-1 max-w-2xl mx-8",
                        div { class: "relative",
                            input {
                                r#type: "text",
                                placeholder: "Search for address, suburb, pharmacy or position...",
                                class: "w-full px-6 py-3 pl-12 pr-16 bg-white/70 dark:bg-gray-800/70 backdrop-blur-lg rounded-2xl border border-white/30 focus:border-blue-400 focus:ring-4 focus:ring-blue-400/20 text-gray-900 dark:text-white placeholder-gray-500 transition-all duration-300 shadow-lg",
                                value: search_query.read().clone(),
                                oninput: move |e| search_query.set(e.value())
                            }
                            
                            // Search Icon
                            div { class: "absolute left-4 top-1/2 transform -translate-y-1/2",
                                span { class: "text-gray-400 text-xl", "🔍" }
                            }
                            
                            // Active Filters Indicator
                            if *active_filters_count.read() > 0 {
                                div { 
                                    class: "absolute right-4 top-1/2 transform -translate-y-1/2 bg-blue-500 text-white px-2 py-1 rounded-full text-xs font-semibold shadow-lg",
                                    "{active_filters_count.read()}"
                                }
                            }
                        }
                    }
                    
                    // Header Controls
                    div { class: "flex items-center space-x-3",
                        // View Mode Toggle
                        button {
                            class: "glass-button p-3 rounded-xl transition-all duration-300 hover:bg-white/20",
                            onclick: move |_| {
                                let current_value = *show_filters_panel.read();
                                show_filters_panel.set(!current_value);
                            },
                            span { class: "text-xl", "⚙️" }
                        }
                        
                        // Location Toggle
                        button {
                            class: if *location_enabled.read() { "glass-button-active p-3 rounded-xl" } else { "glass-button p-3 rounded-xl" },
                            onclick: move |_| {
                                if *location_enabled.read() {
                                    location_enabled.set(false);
                                    user_location.set(None);
                                } else {
                                    show_location_dialog.set(true);
                                }
                            },
                            span { class: "text-xl", "📍" }
                        }
                        
                        // Dark Mode Toggle
                        button {
                            class: if *dark_mode.read() { "glass-button-active p-3 rounded-xl" } else { "glass-button p-3 rounded-xl" },
                            onclick: move |_| {
                                let current_value = *dark_mode.read();
                                dark_mode.set(!current_value);
                            },
                            span { class: "text-xl", if *dark_mode.read() { "🌙" } else { "☀️" } }
                        }
                    }
                }
            }
            
            // Main Content Area
            div { class: "flex-1 flex overflow-hidden",
                
                // Advanced Filters Panel (Collapsible)
                if *show_filters_panel.read() {
                    aside { 
                        class: "w-80 bg-white/70 dark:bg-gray-900/70 backdrop-blur-xl border-r border-white/20 shadow-xl overflow-y-auto",
                        div { class: "p-6 space-y-6",
                            h3 { class: "text-lg font-semibold text-gray-900 dark:text-white flex items-center",
                                span { class: "mr-2 text-xl", "🎛️" }
                                "Advanced Filters"
                            }
                            
                            // Urgent Toggle
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                    "🚨 Priority Filters"
                                }
                                div { class: "flex items-center justify-between glass-control p-3 rounded-xl",
                                    span { class: "text-sm text-gray-600 dark:text-gray-400", "Urgent positions only" }
                                    input {
                                        r#type: "checkbox",
                                        class: "w-5 h-5 text-blue-600 rounded focus:ring-blue-500",
                                        checked: *show_urgent_only.read(),
                                        onchange: move |e| show_urgent_only.set(e.checked())
                                    }
                                }
                            }
                            
                            // Professional Type Filters
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                    "🏥 Professional Types"
                                }
                                div { class: "space-y-2",
                                    FilterToggle {
                                        icon: "🏥",
                                        label: "Hospital Pharmacy",
                                        checked: *show_hospital_jobs.read(),
                                        on_change: move |checked| show_hospital_jobs.set(checked)
                                    }
                                    FilterToggle {
                                        icon: "🏬",
                                        label: "Retail Pharmacy",
                                        checked: *show_retail_jobs.read(),
                                        on_change: move |checked| show_retail_jobs.set(checked)
                                    }
                                    FilterToggle {
                                        icon: "🔬",
                                        label: "Clinical Positions",
                                        checked: *show_clinical_jobs.read(),
                                        on_change: move |checked| show_clinical_jobs.set(checked)
                                    }
                                    FilterToggle {
                                        icon: "💼",
                                        label: "Locum Opportunities",
                                        checked: *show_locum_jobs.read(),
                                        on_change: move |checked| show_locum_jobs.set(checked)
                                    }
                                    FilterToggle {
                                        icon: "⚗️",
                                        label: "Compounding",
                                        checked: *show_compounding_jobs.read(),
                                        on_change: move |checked| show_compounding_jobs.set(checked)
                                    }
                                    FilterToggle {
                                        icon: "🏡",
                                        label: "Aged Care",
                                        checked: *show_aged_care_jobs.read(),
                                        on_change: move |checked| show_aged_care_jobs.set(checked)
                                    }
                                }
                            }
                            
                            // Salary Range Slider
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                    "💰 Hourly Rate Range"
                                }
                                div { class: "glass-control p-4 rounded-xl",
                                    div { class: "flex justify-between items-center mb-3",
                                        span { class: "text-sm text-gray-600", "${salary_range.read().0:.0}/hr" }
                                        span { class: "text-sm text-gray-600", "${salary_range.read().1:.0}/hr" }
                                    }
                                    input {
                                        r#type: "range",
                                        min: "20",
                                        max: "100",
                                        step: "5",
                                        class: "w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer slider",
                                        value: "{salary_range.read().0}",
                                        oninput: move |e| {
                                            if let Ok(val) = e.value().parse::<f64>() {
                                                let current_max = salary_range.read().1;
                                                salary_range.set((val, current_max));
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Time-based Filtering
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                    "⏰ Shift Timing"
                                }
                                div { class: "grid grid-cols-2 gap-2",
                                    TimeFilterButton { label: "All", value: "all", current: time_filter.read().clone(), on_click: move |v| time_filter.set(v) }
                                    TimeFilterButton { label: "Morning", value: "morning", current: time_filter.read().clone(), on_click: move |v| time_filter.set(v) }
                                    TimeFilterButton { label: "Afternoon", value: "afternoon", current: time_filter.read().clone(), on_click: move |v| time_filter.set(v) }
                                    TimeFilterButton { label: "Evening", value: "evening", current: time_filter.read().clone(), on_click: move |v| time_filter.set(v) }
                                }
                            }
                            
                            // Sort Options
                            div { class: "space-y-3",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                    "📊 Sort By"
                                }
                                select {
                                    class: "w-full glass-control p-3 rounded-xl text-gray-900 dark:text-white",
                                    value: sort_by.read().clone(),
                                    onchange: move |e| sort_by.set(e.value()),
                                    option { value: "distance", "Distance from me" }
                                    option { value: "date", "Date posted" }
                                    option { value: "rate", "Hourly rate" }
                                }
                            }
                            
                            // Clear Filters Button
                            button {
                                class: "w-full glass-button-danger p-3 rounded-xl font-medium transition-all duration-300",
                                onclick: move |_| {
                                    search_query.set(String::new());
                                    show_urgent_only.set(false);
                                    show_hospital_jobs.set(true);
                                    show_retail_jobs.set(true);
                                    show_clinical_jobs.set(true);
                                    show_locum_jobs.set(true);
                                    show_compounding_jobs.set(true);
                                    show_aged_care_jobs.set(true);
                                    salary_range.set((20.0, 100.0));
                                    time_filter.set("all".to_string());
                                    sort_by.set("distance".to_string());
                                },
                                span { class: "mr-2", "🗑️" }
                                "Clear All Filters"
                            }
                        }
                    }
                }
                
                // Job List Sidebar
                aside { 
                    class: "w-96 bg-white/70 dark:bg-gray-900/70 backdrop-blur-xl border-r border-white/20 shadow-xl flex flex-col",
                    
                    // Jobs Header with Stats
                    div { class: "p-6 border-b border-white/20",
                        div { class: "flex items-center justify-between mb-4",
                            h2 { class: "text-xl font-bold text-gray-900 dark:text-white flex items-center",
                                span { class: "mr-2 text-2xl", "💼" }
                                "Available Jobs"
                            }
                            div { class: "glass-control px-3 py-1 rounded-full",
                                span { class: "text-sm font-medium text-blue-600", "{filtered_jobs.read().len()}" }
                            }
                        }
                        
                        // Quick Stats
                        div { class: "grid grid-cols-3 gap-3 text-center",
                            StatCard { 
                                icon: "📊",
                                value: format!("{}", total_jobs),
                                label: "Total"
                            }
                            StatCard { 
                                icon: "🚨",
                                value: format!("{}", urgent_jobs),
                                label: "Urgent"
                            }
                            StatCard { 
                                icon: "💰",
                                value: format!("${:.0}", avg_rate),
                                label: "Avg Rate"
                            }
                        }
                    }
                    
                    // Jobs List with Virtual Scrolling
                    div { class: "flex-1 overflow-y-auto",
                        if filtered_jobs.read().is_empty() {
                            div { class: "p-8 text-center",
                                div { class: "text-6xl mb-4", "🔍" }
                                h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-2", "No jobs found" }
                                p { class: "text-gray-600 dark:text-gray-400", "Try adjusting your search criteria or filters" }
                            }
                        } else {
                            div { class: "p-4 space-y-3",
                                for job in filtered_jobs.read().iter() {
                                    EnhancedJobCard {
                                        job: job.clone(),
                                        is_selected: selected_job.read().as_ref().map(|j| j.id) == Some(job.id),
                                        user_location: *user_location.read(),
                                        on_select: move |j| selected_job.set(Some(j)),
                                        on_view_map: move |_j| {
                                            // Focus map on job location
                                        },
                                        on_view_details: move |j| selected_job.set(Some(j))
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Map Container
                main { class: "flex-1 relative",
                    // Map Controls Overlay
                    div { class: "absolute top-4 right-4 z-40 space-y-3",
                        // Map Style Switcher
                        div { class: "glass-control p-2 rounded-xl",
                            MapStyleButton { 
                                icon: "🌞", 
                                active: map_style.read().as_str() == "light",
                                on_click: move |_| map_style.set("light".to_string())
                            }
                            MapStyleButton { 
                                icon: "🗺️", 
                                active: map_style.read().as_str() == "streets",
                                on_click: move |_| map_style.set("streets".to_string())
                            }
                            MapStyleButton { 
                                icon: "🛰️", 
                                active: map_style.read().as_str() == "satellite",
                                on_click: move |_| map_style.set("satellite".to_string())
                            }
                            MapStyleButton { 
                                icon: "🌙", 
                                active: map_style.read().as_str() == "dark",
                                on_click: move |_| map_style.set("dark".to_string())
                            }
                        }
                        
                        // Map Features
                        div { class: "glass-control p-2 rounded-xl space-y-2",
                            FeatureToggle {
                                icon: "🔥",
                                label: "Heatmap",
                                active: *show_heatmap.read(),
                                on_toggle: move |v| show_heatmap.set(v)
                            }
                            FeatureToggle {
                                icon: "🏢",
                                label: "3D Buildings",
                                active: *show_3d_buildings.read(),
                                on_toggle: move |v| show_3d_buildings.set(v)
                            }
                            FeatureToggle {
                                icon: "🚦",
                                label: "Traffic",
                                active: *show_traffic.read(),
                                on_toggle: move |v| show_traffic.set(v)
                            }
                            FeatureToggle {
                                icon: "📍",
                                label: "Clustering",
                                active: *show_clustering.read(),
                                on_toggle: move |v| show_clustering.set(v)
                            }
                        }
                        
                        // User Location Control
                        if *location_enabled.read() {
                            button {
                                class: "glass-button p-3 rounded-xl",
                                onclick: move |_| {
                                    // Center map on user location
                                },
                                span { class: "text-xl", "🎯" }
                            }
                        }
                    }
                    
                    // Map Component - Interactive Visualization
                    div { 
                        class: "w-full h-full relative bg-gradient-to-br from-blue-100 to-green-100 overflow-hidden",
                        
                        // Map Background with Grid Pattern
                        div {
                            class: "absolute inset-0",
                            style: "background-image: url('data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\"><defs><pattern id=\"grid\" width=\"10\" height=\"10\" patternUnits=\"userSpaceOnUse\"><path d=\"M 10 0 L 0 0 0 10\" fill=\"none\" stroke=\"%23e2e8f0\" stroke-width=\"0.5\"/></pattern></defs><rect width=\"100\" height=\"100\" fill=\"url(%23grid)\"/></svg>');",
                        }
                        
                        // Job Markers with Clustering
                        div { class: "absolute inset-0",
                            // Adelaide CBD Cluster
                            div {
                                class: "absolute transform -translate-x-1/2 -translate-y-1/2",
                                style: "top: 45%; left: 48%;",
                                JobCluster {
                                    job_count: filtered_jobs.read().iter().filter(|j| j.state == AustralianState::SouthAustralia).count(),
                                    urgent_count: filtered_jobs.read().iter().filter(|j| j.state == AustralianState::SouthAustralia && j.is_urgent).count(),
                                    avg_rate: {
                                        let jobs_ref = filtered_jobs.read();
                                        let sa_jobs: Vec<_> = jobs_ref.iter().filter(|j| j.state == AustralianState::SouthAustralia).collect();
                                        if sa_jobs.is_empty() { 0.0 } else { sa_jobs.iter().map(|j| j.hourly_rate).sum::<f64>() / sa_jobs.len() as f64 }
                                    },
                                    on_click: move |_| {
                                        if let Some(first_job) = filtered_jobs.read().iter().find(|j| j.state == AustralianState::SouthAustralia) {
                                            selected_job.set(Some(first_job.clone()));
                                        }
                                    }
                                }
                            }
                            
                            // Melbourne Cluster
                            div {
                                class: "absolute transform -translate-x-1/2 -translate-y-1/2",
                                style: "top: 65%; left: 75%;",
                                JobCluster {
                                    job_count: filtered_jobs.read().iter().filter(|j| j.state == AustralianState::Victoria).count(),
                                    urgent_count: filtered_jobs.read().iter().filter(|j| j.state == AustralianState::Victoria && j.is_urgent).count(),
                                    avg_rate: {
                                        let jobs_ref = filtered_jobs.read();
                                        let vic_jobs: Vec<_> = jobs_ref.iter().filter(|j| j.state == AustralianState::Victoria).collect();
                                        if vic_jobs.is_empty() { 0.0 } else { vic_jobs.iter().map(|j| j.hourly_rate).sum::<f64>() / vic_jobs.len() as f64 }
                                    },
                                    on_click: move |_| {
                                        if let Some(first_job) = filtered_jobs.read().iter().find(|j| j.state == AustralianState::Victoria) {
                                            selected_job.set(Some(first_job.clone()));
                                        }
                                    }
                                }
                            }
                            
                            // Sydney Cluster
                            div {
                                class: "absolute transform -translate-x-1/2 -translate-y-1/2",
                                style: "top: 55%; left: 85%;",
                                JobCluster {
                                    job_count: filtered_jobs.read().iter().filter(|j| j.state == AustralianState::NewSouthWales).count(),
                                    urgent_count: filtered_jobs.read().iter().filter(|j| j.state == AustralianState::NewSouthWales && j.is_urgent).count(),
                                    avg_rate: {
                                        let jobs_ref = filtered_jobs.read();
                                        let nsw_jobs: Vec<_> = jobs_ref.iter().filter(|j| j.state == AustralianState::NewSouthWales).collect();
                                        if nsw_jobs.is_empty() { 0.0 } else { nsw_jobs.iter().map(|j| j.hourly_rate).sum::<f64>() / nsw_jobs.len() as f64 }
                                    },
                                    on_click: move |_| {
                                        if let Some(first_job) = filtered_jobs.read().iter().find(|j| j.state == AustralianState::NewSouthWales) {
                                            selected_job.set(Some(first_job.clone()));
                                        }
                                    }
                                }
                            }
                            
                            // Perth Cluster
                            div {
                                class: "absolute transform -translate-x-1/2 -translate-y-1/2",
                                style: "top: 60%; left: 15%;",
                                JobCluster {
                                    job_count: filtered_jobs.read().iter().filter(|j| j.state == AustralianState::WesternAustralia).count(),
                                    urgent_count: filtered_jobs.read().iter().filter(|j| j.state == AustralianState::WesternAustralia && j.is_urgent).count(),
                                    avg_rate: {
                                        let jobs_ref = filtered_jobs.read();
                                        let wa_jobs: Vec<_> = jobs_ref.iter().filter(|j| j.state == AustralianState::WesternAustralia).collect();
                                        if wa_jobs.is_empty() { 0.0 } else { wa_jobs.iter().map(|j| j.hourly_rate).sum::<f64>() / wa_jobs.len() as f64 }
                                    },
                                    on_click: move |_| {
                                        if let Some(first_job) = filtered_jobs.read().iter().find(|j| j.state == AustralianState::WesternAustralia) {
                                            selected_job.set(Some(first_job.clone()));
                                        }
                                    }
                                }
                            }
                            
                            // User Location Marker
                            if let Some(user_loc) = *user_location.read() {
                                div {
                                    class: "absolute transform -translate-x-1/2 -translate-y-1/2",
                                    style: "top: 50%; left: 50%;",
                                    UserLocationMarker {
                                        location: user_loc,
                                        accuracy: 50.0
                                    }
                                }
                            }
                        }
                        
                        // Map Legend
                        div { class: "absolute bottom-4 left-4 glass-control p-4 rounded-xl",
                            h4 { class: "font-semibold text-gray-900 dark:text-white mb-3 flex items-center",
                                span { class: "mr-2", "🗺️" }
                                "Map Legend"
                            }
                            div { class: "space-y-2 text-sm",
                                div { class: "flex items-center space-x-2",
                                    div { class: "w-4 h-4 bg-blue-500 rounded-full" }
                                    span { class: "text-gray-700 dark:text-gray-300", "Job Clusters" }
                                }
                                div { class: "flex items-center space-x-2",
                                    div { class: "w-4 h-4 bg-red-500 rounded-full animate-pulse" }
                                    span { class: "text-gray-700 dark:text-gray-300", "Urgent Positions" }
                                }
                                if *location_enabled.read() {
                                    div { class: "flex items-center space-x-2",
                                        div { class: "w-4 h-4 bg-teal-500 rounded-full" }
                                        span { class: "text-gray-700 dark:text-gray-300", "Your Location" }
                                    }
                                }
                            }
                        }
                        
                        // Map Info Panel
                        div { class: "absolute top-4 left-4 glass-control p-4 rounded-xl",
                            div { class: "space-y-2 text-sm text-gray-700 dark:text-gray-300",
                                p { "🏥 Hospital: {show_hospital_jobs.read()}" }
                                p { "🏬 Retail: {show_retail_jobs.read()}" }
                                p { "🔬 Clinical: {show_clinical_jobs.read()}" }
                                p { "💼 Locum: {show_locum_jobs.read()}" }
                                p { "🎨 Style: {map_style.read()}" }
                                if let Some(loc) = *user_location.read() {
                                    p { "📍 GPS: {loc.0:.3}, {loc.1:.3}" }
                                }
                            }
                        }
                    }
                    
                    // Live Job Feed (if enabled)
                    if *show_live_feed.read() {
                        div { class: "absolute bottom-4 left-4 w-80 glass-control p-4 rounded-xl",
                            h4 { class: "font-semibold text-gray-900 dark:text-white mb-3 flex items-center",
                                span { class: "mr-2 text-red-500 animate-pulse", "🔴" }
                                "Live Job Feed"
                            }
                            div { class: "space-y-2 max-h-40 overflow-y-auto",
                                p { class: "text-sm text-gray-600", "New jobs will appear here in real-time" }
                            }
                        }
                    }
                }
            }
            
            // Enhanced Footer Stats Bar with Real-time Analytics
            footer { 
                class: "bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl border-t border-white/20 shadow-lg",
                div { class: "max-w-7xl mx-auto px-4 py-4",
                    // Main stats row
                    div { class: "flex items-center justify-between mb-2",
                        div { class: "flex items-center space-x-6",
                            // Live job counter with animation
                            div { class: "flex items-center space-x-2 text-sm font-medium",
                                span { class: "text-green-500 animate-pulse", "🟢" }
                                span { class: "text-gray-900 dark:text-white", "Live: {filtered_jobs.read().len()} jobs" }
                                if *live_job_count.read() > 0 {
                                    span { class: "bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs ml-2",
                                        "+{live_job_count.read()} new"
                                    }
                                }
                            }
                            
                            // Location status with accuracy
                            if *location_enabled.read() {
                                div { class: "flex items-center space-x-2 text-sm",
                                    span { class: "text-teal-500", "📍" }
                                    span { class: "text-gray-600 dark:text-gray-400", 
                                        "GPS: {location_source.read()}"
                                    }
                                    if let Some(accuracy) = *location_accuracy.read() {
                                        span { class: "text-xs text-gray-500", "±{accuracy:.0}m" }
                                    }
                                }
                            }
                            
                            // Map visualization status
                            div { class: "flex items-center space-x-2 text-sm text-gray-600 dark:text-gray-400",
                                span { class: "text-blue-500", "🗺️" }
                                span { "{map_style.read()}" }
                                if *show_heatmap.read() {
                                    span { class: "text-red-500 ml-1", "🔥" }
                                }
                                if *show_3d_buildings.read() {
                                    span { class: "text-purple-500 ml-1", "🏢" }
                                }
                            }
                            
                            // Performance metrics
                            div { class: "flex items-center space-x-2 text-sm text-gray-500",
                                span { "⚡" }
                                span { "{render_time.read():.1}ms" }
                            }
                        }
                        
                        // Advanced controls
                        div { class: "flex items-center space-x-3",
                            // Live feed toggle
                            button {
                                class: if *show_live_feed.read() { "glass-button-active px-3 py-1 rounded-full text-sm" } else { "glass-button px-3 py-1 rounded-full text-sm" },
                                onclick: move |_| {
                                    let current_value = *show_live_feed.read();
                                    show_live_feed.set(!current_value);
                                },
                                span { class: "mr-1", "📡" }
                                "Live Feed"
                            }
                            
                            // Auto-refresh with countdown
                            button {
                                class: if *auto_refresh.read() { "glass-button-active px-3 py-1 rounded-full text-sm" } else { "glass-button px-3 py-1 rounded-full text-sm" },
                                onclick: move |_| {
                                    let current_value = *auto_refresh.read();
                                    auto_refresh.set(!current_value);
                                },
                                span { class: "mr-1", if *auto_refresh.read() { "⏸️" } else { "▶️" } }
                                if *auto_refresh.read() { "Auto-refresh" } else { "Paused" }
                            }
                            
                            // Sound toggle
                            button {
                                class: if *sound_enabled.read() { "glass-button-active px-3 py-1 rounded-full text-sm" } else { "glass-button px-3 py-1 rounded-full text-sm" },
                                onclick: move |_| {
                                    let current_value = *sound_enabled.read();
                                    sound_enabled.set(!current_value);
                                },
                                span { if *sound_enabled.read() { "🔊" } else { "🔇" } }
                            }
                        }
                    }
                    
                    // Secondary stats row with detailed analytics
                    div { class: "flex items-center justify-between text-xs text-gray-500 dark:text-gray-400",
                        div { class: "flex items-center space-x-4",
                            span { "📊 Total: {total_jobs}" }
                            span { "🚨 Urgent: {urgent_jobs}" }
                            span { "💰 Avg: ${avg_rate:.0}/hr" }
                            if let Some(nearest) = nearest_jobs.read().first() {
                                span { "🎯 Nearest: {nearest.suburb} ({nearest.distance_km.unwrap_or(0.0):.1}km)" }
                            }
                        }
                        
                        div { class: "flex items-center space-x-4",
                            if *notification_count.read() > 0 {
                                span { class: "text-orange-500", "🔔 {notification_count.read()} notifications" }
                            }
                            span { "🕐 Updated: {last_live_update.read().format(\"%H:%M:%S\")}" }
                            span { "🎛️ Filters: {active_filters_count.read()}" }
                            if *virtual_scrolling.read() {
                                span { "⚡ Virtual scrolling" }
                            }
                        }
                    }
                }
            }
            
            // Location Permission Dialog
            if *show_location_dialog.read() {
                LocationPermissionDialog {
                    on_allow: move |_| {
                        location_enabled.set(true);
                        user_location.set(Some((-34.9285, 138.6007))); // Adelaide default
                        show_location_dialog.set(false);
                    },
                    on_deny: move |_| {
                        show_location_dialog.set(false);
                    }
                }
            }
            
            // Job Detail Modal
            if let Some(job) = selected_job.read().as_ref() {
                JobDetailModal {
                    job: job.clone(),
                    on_close: move |_| selected_job.set(None),
                    on_apply: move |_j| {
                        // Handle job application
                        selected_job.set(None);
                    },
                    on_save: move |_j| {
                        // Handle save job
                    },
                    on_share: move |_j| {
                        // Handle share job
                    }
                }
            }
        }
    }
}

// Helper Components
#[component]
fn FilterToggle(
    icon: String,
    label: String, 
    checked: bool,
    on_change: EventHandler<bool>
) -> Element {
    rsx! {
        div { 
            class: "flex items-center justify-between glass-control p-3 rounded-xl transition-all duration-300 hover:bg-white/30",
            onclick: move |_| on_change.call(!checked),
            div { class: "flex items-center space-x-3 cursor-pointer",
                span { class: "text-lg", "{icon}" }
                span { class: "text-sm text-gray-700 dark:text-gray-300 font-medium", "{label}" }
            }
            input {
                r#type: "checkbox",
                class: "w-5 h-5 text-blue-600 rounded focus:ring-blue-500 cursor-pointer",
                checked: checked,
                onchange: move |e| on_change.call(e.checked())
            }
        }
    }
}

#[component]
fn TimeFilterButton(
    label: String,
    value: String,
    current: String,
    on_click: EventHandler<String>
) -> Element {
    let is_active = current == value;
    
    rsx! {
        button {
            class: if is_active { 
                "glass-button-active p-2 rounded-lg text-sm font-medium transition-all duration-300" 
            } else { 
                "glass-button p-2 rounded-lg text-sm font-medium transition-all duration-300" 
            },
            onclick: move |_| on_click.call(value.clone()),
            "{label}"
        }
    }
}

#[component]
fn StatCard(icon: String, value: String, label: String) -> Element {
    rsx! {
        div { class: "glass-control p-3 rounded-xl text-center",
            div { class: "text-lg mb-1", "{icon}" }
            div { class: "text-lg font-bold text-gray-900 dark:text-white", "{value}" }
            div { class: "text-xs text-gray-500", "{label}" }
        }
    }
}

#[component]
fn EnhancedJobCard(
    job: Job,
    is_selected: bool,
    user_location: Option<(f64, f64)>,
    on_select: EventHandler<Job>,
    on_view_map: EventHandler<Job>,
    on_view_details: EventHandler<Job>
) -> Element {
    let distance = if let (Some(user_loc), Some(lat), Some(lng)) = (user_location, job.latitude, job.longitude) {
        Some(calculate_distance(user_loc, (lat, lng)))
    } else {
        job.distance_km
    };

    // Create clones for each closure
    let job_for_select = job.clone();
    let job_for_view_map = job.clone();
    let job_for_view_details = job.clone();

    rsx! {
        div { 
            class: if is_selected {
                "glass-card-selected p-4 rounded-2xl transition-all duration-300 cursor-pointer border-l-4 border-blue-500 shadow-lg"
            } else {
                "glass-card p-4 rounded-2xl transition-all duration-300 cursor-pointer hover:shadow-xl hover:bg-white/20"
            },
            onclick: move |_| on_select.call(job_for_select.clone()),
            
            // Job Header
            div { class: "flex items-start justify-between mb-3",
                div { class: "flex-1",
                    h3 { class: "font-semibold text-gray-900 dark:text-white text-lg mb-1 leading-tight",
                        "{job.title}"
                    }
                    p { class: "text-sm text-gray-600 dark:text-gray-400 font-medium",
                        "{job.pharmacy_name}"
                    }
                }
                div { class: "text-right",
                    div { class: "text-lg font-bold text-green-600 dark:text-green-400",
                        "${job.hourly_rate:.0}/hr"
                    }
                    if job.is_urgent {
                        span { class: "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200 mt-1",
                            span { class: "mr-1", "🚨" }
                            "Urgent"
                        }
                    }
                }
            }
            
            // Location Info
            div { class: "flex items-center text-sm text-gray-600 dark:text-gray-400 mb-3",
                span { class: "mr-2", "📍" }
                span { "{job.suburb}, {job.state} {job.postcode}" }
                if let Some(dist) = distance {
                    span { class: "ml-auto text-blue-600 dark:text-blue-400 font-medium",
                        "({dist:.1} km)"
                    }
                }
            }
            
            // Schedule Info
            div { class: "flex items-center text-sm text-gray-600 dark:text-gray-400 mb-4",
                span { class: "mr-2", "🕐" }
                span { "{job.start_time} - {job.end_time}" }
                span { class: "mx-2", "•" }
                span { "{job.start_date.format(\"%d/%m/%Y\")}" }
            }
            
            // Action Buttons
            div { class: "flex space-x-2",
                button {
                    class: "flex-1 glass-button py-2 px-3 rounded-lg text-sm font-medium transition-all duration-300",
                    onclick: {
                        move |e| {
                            e.stop_propagation();
                            on_view_map.call(job_for_view_map.clone());
                        }
                    },
                    "View on Map"
                }
                button {
                    class: "flex-1 glass-button-primary py-2 px-3 rounded-lg text-sm font-medium transition-all duration-300",
                    onclick: {
                        move |e| {
                            e.stop_propagation();
                            on_view_details.call(job_for_view_details.clone());
                        }
                    },
                    "View Details"
                }
            }
        }
    }
}

#[component]
fn MapStyleButton(
    icon: String,
    active: bool,
    on_click: EventHandler<()>
) -> Element {
    rsx! {
        button {
            class: if active {
                "p-2 rounded-lg bg-blue-500 text-white shadow-lg transition-all duration-300"
            } else {
                "p-2 rounded-lg bg-white/50 text-gray-700 hover:bg-white/70 transition-all duration-300"
            },
            onclick: move |_| on_click.call(()),
            "{icon}"
        }
    }
}

#[component]
fn FeatureToggle(
    icon: String,
    label: String,
    active: bool,
    on_toggle: EventHandler<bool>
) -> Element {
    rsx! {
        button {
            class: if active {
                "flex items-center space-x-2 w-full p-2 rounded-lg bg-blue-500 text-white shadow-lg transition-all duration-300"
            } else {
                "flex items-center space-x-2 w-full p-2 rounded-lg bg-white/50 text-gray-700 hover:bg-white/70 transition-all duration-300"
            },
            onclick: move |_| on_toggle.call(!active),
            span { "{icon}" }
            span { class: "text-xs font-medium", "{label}" }
        }
    }
}

#[component]
fn JobCluster(
    job_count: usize,
    urgent_count: usize,
    avg_rate: f64,
    on_click: EventHandler<()>
) -> Element {
    rsx! {
        div { 
            class: "relative group cursor-pointer",
            onclick: move |_| on_click.call(()),
            
            // Cluster background with glass effect
            div {
                class: format!(
                    "w-16 h-16 rounded-full backdrop-blur-xl border border-white/20 shadow-xl flex items-center justify-center transform transition-all duration-300 hover:scale-110 hover:shadow-2xl {}",
                    if urgent_count > 0 { "bg-red-500/70" } else { "bg-white/70" }
                ),
                
                div {
                    class: "text-center",
                    div {
                        class: if urgent_count > 0 { "text-lg font-bold text-white" } else { "text-lg font-bold text-blue-600" },
                        "{job_count}"
                    }
                    div {
                        class: if urgent_count > 0 { "text-xs text-white" } else { "text-xs text-gray-600" },
                        "jobs"
                    }
                }
            }
            
            // Pulse animation for urgent jobs
            if urgent_count > 0 {
                div {
                    class: "absolute inset-0 rounded-full bg-red-400 animate-ping opacity-30"
                }
            }
            
            // Tooltip
            div {
                class: "absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none",
                div {
                    class: "glass-control p-2 rounded-lg text-xs text-center whitespace-nowrap",
                    p { class: "font-semibold", "{job_count} positions" }
                    if urgent_count > 0 {
                        p { class: "text-red-600", "{urgent_count} urgent" }
                    }
                    if avg_rate > 0.0 {
                        p { class: "text-green-600", "Avg: ${avg_rate:.0}/hr" }
                    }
                }
            }
        }
    }
}

#[component]
fn UserLocationMarker(
    location: (f64, f64),
    accuracy: f64
) -> Element {
    rsx! {
        div { class: "relative",
            // Accuracy circle
            div {
                class: "absolute inset-0 rounded-full border-2 border-teal-300 opacity-20",
                style: "width: {accuracy}px; height: {accuracy}px; transform: translate(-50%, -50%);"
            }
            
            // Location marker
            div {
                class: "w-6 h-6 rounded-full bg-teal-500/90 backdrop-blur-sm border-2 border-white shadow-lg relative",
                
                // Center dot
                div {
                    class: "absolute inset-2 rounded-full bg-white"
                }
                
                // Pulse animation
                div {
                    class: "absolute inset-0 rounded-full bg-teal-400 animate-ping opacity-75"
                }
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
        div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
            div { class: "glass-card p-8 rounded-2xl max-w-md mx-4 shadow-2xl",
                div { class: "text-center",
                    div { class: "text-6xl mb-4", "📍" }
                    h3 { class: "text-xl font-bold text-gray-900 dark:text-white mb-2",
                        "Enable Location Services"
                    }
                    p { class: "text-gray-600 dark:text-gray-400 mb-6",
                        "Allow location access to find jobs near you and get accurate distances."
                    }
                    div { class: "flex space-x-3",
                        button {
                            class: "flex-1 glass-button py-3 px-4 rounded-xl font-medium",
                            onclick: move |_| on_deny.call(()),
                            "Not Now"
                        }
                        button {
                            class: "flex-1 glass-button-primary py-3 px-4 rounded-xl font-medium",
                            onclick: move |_| on_allow.call(()),
                            "Allow Location"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn JobDetailModal(
    job: Job,
    on_close: EventHandler<()>,
    on_apply: EventHandler<Job>,
    on_save: EventHandler<Job>,
    on_share: EventHandler<Job>
) -> Element {
    rsx! {
        div { 
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4",
            onclick: move |_| on_close.call(()),
            
            div { 
                class: "glass-card p-8 rounded-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto shadow-2xl",
                onclick: move |e| e.stop_propagation(),
                
                // Modal Header
                div { class: "flex items-start justify-between mb-6",
                    div {
                        h2 { class: "text-2xl font-bold text-gray-900 dark:text-white mb-2",
                            "{job.title}"
                        }
                        p { class: "text-lg text-gray-600 dark:text-gray-400",
                            "{job.pharmacy_name}"
                        }
                    }
                    button {
                        class: "glass-button p-2 rounded-xl",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }
                
                // Job Details
                div { class: "space-y-6 mb-8",
                    // Key Info
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "glass-control p-4 rounded-xl",
                            div { class: "text-2xl font-bold text-green-600 mb-1", "${job.hourly_rate:.0}/hr" }
                            div { class: "text-sm text-gray-500", "Hourly Rate" }
                        }
                        div { class: "glass-control p-4 rounded-xl",
                            div { class: "text-lg font-semibold text-gray-900 dark:text-white mb-1", 
                                match job.job_type {
                                    JobType::Pharmacist => "👨‍⚕️ Pharmacist",
                                    JobType::Intern => "🎓 Intern",
                                    JobType::PharmacyAssistant => "🏪 Assistant",
                                    JobType::PharmacyTechnician => "🔬 Technician",
                                    _ => "💼 Position"
                                }
                            }
                            div { class: "text-sm text-gray-500", "Position Type" }
                        }
                    }
                    
                    // Description
                    div { class: "glass-control p-4 rounded-xl",
                        h4 { class: "font-semibold text-gray-900 dark:text-white mb-3", "📄 Job Description" }
                        p { class: "text-gray-600 dark:text-gray-400 leading-relaxed", "{job.description}" }
                    }
                    
                    // Location & Schedule
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        div { class: "glass-control p-4 rounded-xl",
                            h4 { class: "font-semibold text-gray-900 dark:text-white mb-3", "📍 Location" }
                            p { class: "text-gray-600 dark:text-gray-400", "{job.address}" }
                            p { class: "text-gray-600 dark:text-gray-400", "{job.suburb}, {job.state} {job.postcode}" }
                        }
                        div { class: "glass-control p-4 rounded-xl",
                            h4 { class: "font-semibold text-gray-900 dark:text-white mb-3", "🕐 Schedule" }
                            p { class: "text-gray-600 dark:text-gray-400", "{job.start_time} - {job.end_time}" }
                            p { class: "text-gray-600 dark:text-gray-400", 
                                "{job.start_date.format(\"%d %B %Y\")} - {job.end_date.format(\"%d %B %Y\")}"
                            }
                        }
                    }
                }
                
                // Action Buttons
                div { class: "flex flex-col sm:flex-row gap-3",
                    button {
                        class: "flex-1 bg-gradient-to-r from-blue-500 to-indigo-600 text-white py-4 px-6 rounded-xl font-semibold shadow-lg hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1",
                        onclick: {
                            let job_clone = job.clone();
                            move |_| on_apply.call(job_clone.clone())
                        },
                        span { class: "mr-2", "📝" }
                        "Apply Now"
                    }
                    div { class: "flex gap-3",
                        button {
                            class: "glass-button py-4 px-6 rounded-xl font-medium",
                            onclick: {
                                let job_clone = job.clone();
                                move |_| on_save.call(job_clone.clone())
                            },
                            span { class: "mr-2", "❤️" }
                            "Save"
                        }
                        button {
                            class: "glass-button py-4 px-6 rounded-xl font-medium",
                            onclick: {
                                let job_clone = job.clone();
                                move |_| on_share.call(job_clone.clone())
                            },
                            span { class: "mr-2", "📤" }
                            "Share"
                        }
                    }
                }
            }
        }
    }
}

// Utility function to calculate distance between two coordinates
fn calculate_distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    let (lat1, lon1) = point1;
    let (lat2, lon2) = point2;
    
    let earth_radius = 6371.0; // Earth's radius in kilometres
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) +
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    earth_radius * c
}

// Comprehensive mock data with all Australian states
fn get_comprehensive_mock_jobs() -> Vec<Job> {
    vec![
        Job {
            id: JobId::new(),
            title: "Senior Hospital Pharmacist - Royal Adelaide".to_string(),
            description: "Join our clinical pharmacy team at Royal Adelaide Hospital. Lead medication management, provide clinical consultations, and mentor junior staff. AHPRA registration and hospital experience required.".to_string(),
            pharmacy_name: "Royal Adelaide Hospital Pharmacy".to_string(),
            hourly_rate: 58.50,
            address: "Port Road, Adelaide".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode::new("5000").unwrap(),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9285),
            longitude: Some(138.6007),
            start_date: Utc::now() + chrono::Duration::days(5),
            end_date: Utc::now() + chrono::Duration::days(365),
            start_time: "07:00".to_string(),
            end_time: "15:30".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: Some(2.1),
            created_at: Utc::now() - chrono::Duration::days(2),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Retail Pharmacist - Rundle Mall".to_string(),
            description: "Busy city pharmacy seeking experienced retail pharmacist. Modern dispensary, excellent customer service focus. Weekend availability preferred.".to_string(),
            pharmacy_name: "City Central Pharmacy".to_string(),
            hourly_rate: 45.00,
            address: "Rundle Mall, Adelaide".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode::new("5000").unwrap(),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9245),
            longitude: Some(138.6012),
            start_date: Utc::now() + chrono::Duration::days(14),
            end_date: Utc::now() + chrono::Duration::days(90),
            start_time: "08:30".to_string(),
            end_time: "17:30".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(1.8),
            created_at: Utc::now() - chrono::Duration::days(5),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Clinical Pharmacy Specialist - Melbourne".to_string(),
            description: "Specialist clinical pharmacist for oncology ward. Provide expert medication therapy management for cancer patients. Advanced qualifications required.".to_string(),
            pharmacy_name: "Peter MacCallum Cancer Centre".to_string(),
            hourly_rate: 68.00,
            address: "305 Grattan Street, Melbourne".to_string(),
            suburb: "Melbourne".to_string(),
            postcode: Postcode::new("3000").unwrap(),
            state: AustralianState::Victoria,
            latitude: Some(-37.8136),
            longitude: Some(144.9631),
            start_date: Utc::now() + chrono::Duration::days(21),
            end_date: Utc::now() + chrono::Duration::days(180),
            start_time: "08:00".to_string(),
            end_time: "16:30".to_string(),
            job_type: JobType::PharmacyTechnician,
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: Some(727.0),
            created_at: Utc::now() - chrono::Duration::days(1),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Pharmacy Intern - Gold Coast".to_string(),
            description: "12-month internship program at busy community pharmacy. Comprehensive training in dispensing, counselling, and pharmacy management. Recent graduates welcome.".to_string(),
            pharmacy_name: "Surfers Paradise Pharmacy".to_string(),
            hourly_rate: 32.50,
            address: "Cavill Avenue, Surfers Paradise".to_string(),
            suburb: "Surfers Paradise".to_string(),
            postcode: Postcode::new("4217").unwrap(),
            state: AustralianState::Queensland,
            latitude: Some(-28.0023),
            longitude: Some(153.4145),
            start_date: Utc::now() + chrono::Duration::days(28),
            end_date: Utc::now() + chrono::Duration::days(393),
            start_time: "08:30".to_string(),
            end_time: "17:00".to_string(),
            job_type: JobType::Intern,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(1247.0),
            created_at: Utc::now() - chrono::Duration::days(3),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Locum Pharmacist - Perth Metro".to_string(),
            description: "Flexible locum opportunities across Perth metropolitan area. Cover holiday periods and staff shortages. Reliable transport essential.".to_string(),
            pharmacy_name: "Perth Locum Network".to_string(),
            hourly_rate: 75.00,
            address: "Various locations, Perth".to_string(),
            suburb: "Perth".to_string(),
            postcode: Postcode::new("6000").unwrap(),
            state: AustralianState::WesternAustralia,
            latitude: Some(-31.9505),
            longitude: Some(115.8605),
            start_date: Utc::now() + chrono::Duration::days(7),
            end_date: Utc::now() + chrono::Duration::days(365),
            start_time: "08:00".to_string(),
            end_time: "18:00".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: Some(2130.0),
            created_at: Utc::now() - chrono::Duration::hours(6),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Compounding Specialist - Sydney".to_string(),
            description: "Specialist compounding pharmacist for high-end compounding pharmacy. Experience in sterile preparations and quality assurance required.".to_string(),
            pharmacy_name: "Precision Compounding Pharmacy".to_string(),
            hourly_rate: 62.00,
            address: "Double Bay, Sydney".to_string(),
            suburb: "Double Bay".to_string(),
            postcode: Postcode::new("2028").unwrap(),
            state: AustralianState::NewSouthWales,
            latitude: Some(-33.8791),
            longitude: Some(151.2383),
            start_date: Utc::now() + chrono::Duration::days(10),
            end_date: Utc::now() + chrono::Duration::days(120),
            start_time: "07:30".to_string(),
            end_time: "16:00".to_string(),
            job_type: JobType::PharmacyTechnician,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(1374.0),
            created_at: Utc::now() - chrono::Duration::days(4),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Aged Care Consultant Pharmacist - Tasmania".to_string(),
            description: "Provide pharmaceutical care to residential aged care facilities across Hobart. Travel required, company vehicle provided.".to_string(),
            pharmacy_name: "Tasmanian Aged Care Pharmacy Services".to_string(),
            hourly_rate: 85.00,
            address: "Hobart, Tasmania".to_string(),
            suburb: "Hobart".to_string(),
            postcode: Postcode::new("7000").unwrap(),
            state: AustralianState::Tasmania,
            latitude: Some(-42.8821),
            longitude: Some(147.3272),
            start_date: Utc::now() + chrono::Duration::days(35),
            end_date: Utc::now() + chrono::Duration::days(200),
            start_time: "08:00".to_string(),
            end_time: "17:00".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(1523.0),
            created_at: Utc::now() - chrono::Duration::days(8),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Pharmacy Assistant - Canberra Central".to_string(),
            description: "Part-time pharmacy assistant for busy Canberra pharmacy. Perfect for students, flexible hours available. Training provided.".to_string(),
            pharmacy_name: "Capital Chemist Civic".to_string(),
            hourly_rate: 28.75,
            address: "Civic Centre, Canberra".to_string(),
            suburb: "Canberra".to_string(),
            postcode: Postcode::new("2601").unwrap(),
            state: AustralianState::AustralianCapitalTerritory,
            latitude: Some(-35.2809),
            longitude: Some(149.1300),
            start_date: Utc::now() + chrono::Duration::days(3),
            end_date: Utc::now() + chrono::Duration::days(180),
            start_time: "09:00".to_string(),
            end_time: "14:00".to_string(),
            job_type: JobType::PharmacyAssistant,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(419.0),
            created_at: Utc::now() - chrono::Duration::days(1),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Remote Area Pharmacist - Alice Springs".to_string(),
            description: "Challenging opportunity in Central Australia. Provide pharmacy services to remote communities. Accommodation and travel allowances included.".to_string(),
            pharmacy_name: "Central Australian Health Service".to_string(),
            hourly_rate: 95.00,
            address: "Gap Road, Alice Springs".to_string(),
            suburb: "Alice Springs".to_string(),
            postcode: Postcode::new("0870").unwrap(),
            state: AustralianState::NorthernTerritory,
            latitude: Some(-23.6980),
            longitude: Some(133.8807),
            start_date: Utc::now() + chrono::Duration::days(42),
            end_date: Utc::now() + chrono::Duration::days(365),
            start_time: "08:00".to_string(),
            end_time: "17:00".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: Some(1533.0),
            created_at: Utc::now() - chrono::Duration::hours(18),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
        Job {
            id: JobId::new(),
            title: "Weekend Relief Pharmacist - Adelaide Hills".to_string(),
            description: "Weekend relief pharmacist for scenic Adelaide Hills pharmacy. Beautiful location, supportive team, competitive rates.".to_string(),
            pharmacy_name: "Stirling Hills Pharmacy".to_string(),
            hourly_rate: 52.00,
            address: "Mount Barker Road, Stirling".to_string(),
            suburb: "Stirling".to_string(),
            postcode: Postcode::new("5152").unwrap(),
            state: AustralianState::SouthAustralia,
            latitude: Some(-35.0154),
            longitude: Some(138.7145),
            start_date: Utc::now() + chrono::Duration::days(9),
            end_date: Utc::now() + chrono::Duration::days(90),
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            job_type: JobType::Pharmacist,
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(18.2),
            created_at: Utc::now() - chrono::Duration::days(6),
            updated_at: Utc::now(),
            created_by: UserId::new(),
        },
    ]
}

// Advanced Map Components for Enhanced Functionality

#[component]
fn HeatmapOverlay(
    heatmap_type: String,
    jobs: Vec<Job>,
    on_type_change: EventHandler<String>
) -> Element {
    rsx! {
        div { class: "absolute inset-0 pointer-events-none",
            // Heatmap visualization based on type
            match heatmap_type.as_str() {
                "density" => rsx! {
                    DensityHeatmap { jobs: jobs.clone() }
                },
                "salary" => rsx! {
                    SalaryHeatmap { jobs: jobs.clone() }
                },
                "time" => rsx! {
                    TimeHeatmap { jobs: jobs.clone() }
                },
                _ => rsx! { div {} }
            }
            
            // Heatmap Control Panel
            div { class: "absolute top-4 left-4 glass-control p-3 rounded-xl pointer-events-auto",
                h4 { class: "text-sm font-semibold text-gray-900 dark:text-white mb-2",
                    "🔥 Heatmap Visualization"
                }
                div { class: "space-y-2",
                    HeatmapControlButton {
                        label: "Job Density",
                        icon: "📊",
                        active: heatmap_type.as_str() == "density",
                        on_click: move |_| on_type_change.call("density".to_string())
                    }
                    HeatmapControlButton {
                        label: "Salary Ranges",
                        icon: "💰",
                        active: heatmap_type.as_str() == "salary", 
                        on_click: move |_| on_type_change.call("salary".to_string())
                    }
                    HeatmapControlButton {
                        label: "Shift Times",
                        icon: "⏰",
                        active: heatmap_type.as_str() == "time",
                        on_click: move |_| on_type_change.call("time".to_string())
                    }
                }
            }
        }
    }
}

#[component]
fn DensityHeatmap(jobs: Vec<Job>) -> Element {
    rsx! {
        div { class: "absolute inset-0",
            // High density areas (Adelaide CBD)
            div {
                class: "absolute rounded-full opacity-20 bg-gradient-radial from-red-500 via-orange-400 to-transparent",
                style: "top: 40%; left: 45%; width: 120px; height: 120px; transform: translate(-50%, -50%);"
            }
            // Medium density areas (Melbourne)
            div {
                class: "absolute rounded-full opacity-15 bg-gradient-radial from-orange-400 via-yellow-400 to-transparent",
                style: "top: 60%; left: 70%; width: 80px; height: 80px; transform: translate(-50%, -50%);"
            }
            // Low density areas (Perth)
            div {
                class: "absolute rounded-full opacity-10 bg-gradient-radial from-yellow-400 via-green-400 to-transparent",
                style: "top: 55%; left: 15%; width: 60px; height: 60px; transform: translate(-50%, -50%);"
            }
        }
    }
}

#[component]
fn SalaryHeatmap(jobs: Vec<Job>) -> Element {
    rsx! {
        div { class: "absolute inset-0",
            // High salary areas (Sydney, Melbourne)
            div {
                class: "absolute rounded-full opacity-25 bg-gradient-radial from-green-500 via-emerald-400 to-transparent",
                style: "top: 50%; left: 80%; width: 100px; height: 100px; transform: translate(-50%, -50%);"
            }
            div {
                class: "absolute rounded-full opacity-20 bg-gradient-radial from-green-400 via-teal-400 to-transparent",
                style: "top: 60%; left: 70%; width: 90px; height: 90px; transform: translate(-50%, -50%);"
            }
            // Medium salary areas (Adelaide)
            div {
                class: "absolute rounded-full opacity-15 bg-gradient-radial from-blue-400 via-indigo-400 to-transparent",
                style: "top: 45%; left: 48%; width: 70px; height: 70px; transform: translate(-50%, -50%);"
            }
        }
    }
}

#[component]
fn TimeHeatmap(jobs: Vec<Job>) -> Element {
    rsx! {
        div { class: "absolute inset-0",
            // Morning shifts - cooler colors
            div {
                class: "absolute rounded-full opacity-15 bg-gradient-radial from-blue-400 via-cyan-300 to-transparent",
                style: "top: 40%; left: 30%; width: 80px; height: 80px; transform: translate(-50%, -50%);"
            }
            // Afternoon shifts - warmer colors
            div {
                class: "absolute rounded-full opacity-20 bg-gradient-radial from-orange-400 via-yellow-400 to-transparent",
                style: "top: 50%; left: 60%; width: 100px; height: 100px; transform: translate(-50%, -50%);"
            }
            // Evening shifts - purple/pink colors
            div {
                class: "absolute rounded-full opacity-18 bg-gradient-radial from-purple-400 via-pink-400 to-transparent",
                style: "top: 55%; left: 80%; width: 90px; height: 90px; transform: translate(-50%, -50%);"
            }
        }
    }
}

#[component]
fn HeatmapControlButton(
    label: String,
    icon: String,
    active: bool,
    on_click: EventHandler<()>
) -> Element {
    rsx! {
        button {
            class: if active {
                "w-full flex items-center space-x-2 p-2 rounded-lg bg-blue-500 text-white text-sm font-medium transition-all duration-300"
            } else {
                "w-full flex items-center space-x-2 p-2 rounded-lg bg-white/50 text-gray-700 hover:bg-white/70 text-sm font-medium transition-all duration-300"
            },
            onclick: move |_| on_click.call(()),
            span { "{icon}" }
            span { "{label}" }
        }
    }
}

#[component]
fn HeatmapTypeButton(
    label: String,
    icon: String,
    active: bool,
    on_click: EventHandler<()>
) -> Element {
    rsx! {
        button {
            class: if active {
                "flex items-center space-x-1 px-2 py-1 rounded text-xs bg-blue-500 text-white font-medium"
            } else {
                "flex items-center space-x-1 px-2 py-1 rounded text-xs bg-white/50 text-gray-700 hover:bg-white/70 font-medium"
            },
            onclick: move |_| on_click.call(()),
            span { class: "text-xs", "{icon}" }
            span { "{label}" }
        }
    }
}

#[component]
fn LiveJobFeedPanel(
    auto_refresh: bool,
    sound_enabled: bool,
    jobs: Vec<Job>,
    on_job_select: EventHandler<Job>
) -> Element {
    let mut recent_jobs = use_signal(|| Vec::<Job>::new());
    let mut last_update = use_signal(|| chrono::Utc::now());
    
    // Simulate real-time job updates
    use_effect(move || {
        if auto_refresh {
            // Add latest jobs to recent feed
            let mut recent = recent_jobs.read().clone();
            let latest_jobs: Vec<Job> = jobs.iter()
                .filter(|job| job.created_at > *last_update.read())
                .cloned()
                .collect();
            
            if !latest_jobs.is_empty() {
                recent.extend(latest_jobs);
                recent.truncate(5); // Keep only 5 most recent
                recent_jobs.set(recent);
                last_update.set(chrono::Utc::now());
            }
        }
    });

    rsx! {
        div { class: "absolute bottom-4 left-4 w-80 glass-control p-4 rounded-xl",
            div { class: "flex items-center justify-between mb-3",
                h4 { class: "font-semibold text-gray-900 dark:text-white flex items-center",
                    span { class: "mr-2 text-red-500 animate-pulse", "🔴" }
                    "Live Job Feed"
                }
                div { class: "flex items-center space-x-2",
                    if sound_enabled {
                        span { class: "text-xs text-green-500", "🔊" }
                    } else {
                        span { class: "text-xs text-gray-400", "🔇" }
                    }
                    if auto_refresh {
                        span { class: "text-xs text-blue-500 animate-pulse", "⚡" }
                    } else {
                        span { class: "text-xs text-gray-400", "⏸️" }
                    }
                }
            }
            
            div { class: "space-y-2 max-h-40 overflow-y-auto",
                if recent_jobs.read().is_empty() {
                    div { class: "text-center py-4",
                        div { class: "text-2xl mb-2", "📡" }
                        p { class: "text-sm text-gray-600", "Monitoring for new jobs..." }
                        p { class: "text-xs text-gray-500", "Real-time updates will appear here" }
                    }
                } else {
                    for job in recent_jobs.read().iter() {
                        LiveJobItem {
                            job: job.clone(),
                            on_select: on_job_select
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LiveJobItem(
    job: Job,
    on_select: EventHandler<Job>
) -> Element {
    rsx! {
        div { 
            class: "glass-control p-3 rounded-lg cursor-pointer hover:bg-white/30 transition-all duration-300 border-l-2 border-green-400",
            onclick: move |_| on_select.call(job.clone()),
            
            div { class: "flex items-start justify-between",
                div { class: "flex-1",
                    h5 { class: "text-sm font-semibold text-gray-900 dark:text-white mb-1",
                        "{job.title}"
                    }
                    p { class: "text-xs text-gray-600 dark:text-gray-400",
                        "{job.pharmacy_name}"
                    }
                    p { class: "text-xs text-gray-500",
                        "📍 {job.suburb} • ${job.hourly_rate:.0}/hr"
                    }
                }
                div { class: "text-right",
                    span { class: "text-xs text-green-600 font-medium", "NEW" }
                    if job.is_urgent {
                        div { class: "text-xs text-red-600 font-medium mt-1", "🚨 URGENT" }
                    }
                }
            }
        }
    }
}

#[component]
fn CommuteCalculatorPanel(
    user_location: Option<(f64, f64)>,
    selected_job: Option<Job>,
    on_close: EventHandler<()>
) -> Element {
    let mut transport_mode = use_signal(|| "driving".to_string());
    let mut commute_time = use_signal(|| None::<f64>);
    let mut commute_cost = use_signal(|| None::<f64>);
    let mut route_options = use_signal(|| Vec::<String>::new());

    rsx! {
        div { class: "absolute bottom-4 right-4 w-80 glass-control p-4 rounded-xl",
            div { class: "flex items-center justify-between mb-3",
                h4 { class: "font-semibold text-gray-900 dark:text-white flex items-center",
                    span { class: "mr-2", "🚗" }
                    "Commute Calculator"
                }
                button {
                    class: "glass-button p-1 rounded",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }
            
            if let (Some(_user_loc), Some(job)) = (user_location, selected_job.as_ref()) {
                div { class: "space-y-3",
                    // Destination
                    div { class: "glass-control p-2 rounded",
                        p { class: "text-xs text-gray-500", "To:" }
                        p { class: "text-sm font-medium", "{job.pharmacy_name}" }
                        p { class: "text-xs text-gray-600", "{job.address}" }
                    }
                    
                    // Transport Mode Selection
                    div {
                        p { class: "text-sm font-medium mb-2", "Transport Mode:" }
                        div { class: "grid grid-cols-2 gap-2",
                            TransportModeButton {
                                mode: "driving",
                                icon: "🚗",
                                label: "Drive",
                                active: transport_mode.read().as_str() == "driving",
                                on_select: move |mode| transport_mode.set(mode)
                            }
                            TransportModeButton {
                                mode: "transit",
                                icon: "🚌",
                                label: "Transit",
                                active: transport_mode.read().as_str() == "transit",
                                on_select: move |mode| transport_mode.set(mode)
                            }
                            TransportModeButton {
                                mode: "walking",
                                icon: "🚶",
                                label: "Walk",
                                active: transport_mode.read().as_str() == "walking",
                                on_select: move |mode| transport_mode.set(mode)
                            }
                            TransportModeButton {
                                mode: "cycling",
                                icon: "🚴",
                                label: "Bike",
                                active: transport_mode.read().as_str() == "cycling",
                                on_select: move |mode| transport_mode.set(mode)
                            }
                        }
                    }
                    
                    // Commute Results
                    div { class: "glass-control p-3 rounded",
                        CommuteResults {
                            distance: job.distance_km.unwrap_or(0.0),
                            mode: transport_mode.read().clone(),
                            time: 25.0, // Mock calculated time
                            cost: 8.50  // Mock calculated cost
                        }
                    }
                    
                    // Action Buttons
                    div { class: "flex space-x-2",
                        button {
                            class: "flex-1 glass-button py-2 px-3 rounded text-sm",
                            onclick: move |_| {
                                // Get directions functionality
                            },
                            "Get Directions"
                        }
                        button {
                            class: "flex-1 glass-button-primary py-2 px-3 rounded text-sm",
                            onclick: move |_| {
                                // Save route functionality
                            },
                            "Save Route"
                        }
                    }
                }
            } else {
                div { class: "text-center py-4",
                    div { class: "text-3xl mb-2", "📍" }
                    p { class: "text-sm text-gray-600", "Select a job to calculate commute" }
                    p { class: "text-xs text-gray-500", "Location services required" }
                }
            }
        }
    }
}

#[component]
fn TransportModeButton(
    mode: String,
    icon: String,
    label: String,
    active: bool,
    on_select: EventHandler<String>
) -> Element {
    rsx! {
        button {
            class: if active {
                "glass-button-active p-2 rounded text-xs font-medium flex flex-col items-center space-y-1"
            } else {
                "glass-button p-2 rounded text-xs font-medium flex flex-col items-center space-y-1 hover:bg-white/30"
            },
            onclick: move |_| on_select.call(mode.clone()),
            span { class: "text-lg", "{icon}" }
            span { "{label}" }
        }
    }
}

#[component]
fn CommuteResults(
    distance: f64,
    mode: String,
    time: f64,
    cost: f64
) -> Element {
    rsx! {
        div { class: "space-y-2",
            h5 { class: "text-sm font-semibold text-gray-900 dark:text-white", "Route Summary" }
            
            div { class: "grid grid-cols-2 gap-3 text-center",
                div {
                    div { class: "text-lg font-bold text-blue-600", "{distance:.1} km" }
                    div { class: "text-xs text-gray-500", "Distance" }
                }
                div {
                    div { class: "text-lg font-bold text-green-600", "{time:.0} min" }
                    div { class: "text-xs text-gray-500", "Time" }
                }
            }
            
            if mode == "driving" || mode == "transit" {
                div { class: "text-center",
                    div { class: "text-lg font-bold text-orange-600", "${cost:.2}" }
                    div { class: "text-xs text-gray-500", 
                        if mode == "driving" { "Fuel + Parking" } else { "Transit Fare" }
                    }
                }
            }
            
            // Environmental impact
            div { class: "pt-2 border-t border-white/20",
                div { class: "flex items-center justify-between text-xs",
                    span { class: "text-gray-600", "CO₂ Impact:" }
                    span { 
                        class: if mode == "walking" || mode == "cycling" { "text-green-600" } else { "text-orange-600" },
                        match mode.as_str() {
                            "walking" | "cycling" => "0kg CO₂",
                            "transit" => "2.1kg CO₂", 
                            _ => "4.8kg CO₂"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DirectionsPanel(
    user_location: Option<(f64, f64)>,
    selected_job: Option<Job>,
    on_close: EventHandler<()>
) -> Element {
    let _route_loaded = use_signal(|| false);
    
    rsx! {
        div { class: "absolute top-1/2 left-4 w-80 glass-control p-4 rounded-xl transform -translate-y-1/2",
            div { class: "flex items-center justify-between mb-3",
                h4 { class: "font-semibold text-gray-900 dark:text-white flex items-center",
                    span { class: "mr-2", "🧭" }
                    "Turn-by-Turn Directions"
                }
                button {
                    class: "glass-button p-1 rounded",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }
            
            if let (Some(_user_loc), Some(job)) = (user_location, selected_job.as_ref()) {
                div { class: "space-y-3",
                    // Route header
                    div { class: "glass-control p-3 rounded",
                        div { class: "flex items-center justify-between mb-2",
                            div {
                                p { class: "text-sm font-medium", "To: {job.pharmacy_name}" }
                                p { class: "text-xs text-gray-600", "{job.address}" }
                            }
                            div { class: "text-right",
                                p { class: "text-sm font-bold text-blue-600", "12.5 km" }
                                p { class: "text-xs text-green-600", "18 min" }
                            }
                        }
                        
                        div { class: "flex items-center space-x-2 text-xs text-gray-500",
                            span { "🚗" }
                            span { "Fastest route • Light traffic" }
                        }
                    }
                    
                    // Turn-by-turn directions
                    div { class: "space-y-2 max-h-60 overflow-y-auto",
                        DirectionStep {
                            step: 1,
                            instruction: "Head northeast on King William Street",
                            distance: "500m",
                            icon: "⬆️"
                        }
                        DirectionStep {
                            step: 2,
                            instruction: "Turn right onto Rundle Street",
                            distance: "1.2km",
                            icon: "➡️"
                        }
                        DirectionStep {
                            step: 3,
                            instruction: "Continue onto Magill Road",
                            distance: "3.8km",
                            icon: "⬆️"
                        }
                        DirectionStep {
                            step: 4,
                            instruction: "Turn left onto Portrush Road",
                            distance: "2.1km",
                            icon: "⬅️"
                        }
                        DirectionStep {
                            step: 5,
                            instruction: "Turn right - Destination on right",
                            distance: "200m",
                            icon: "🏁"
                        }
                    }
                    
                    // Action buttons
                    div { class: "flex space-x-2",
                        button {
                            class: "flex-1 glass-button py-2 px-3 rounded text-sm",
                            onclick: move |_| {
                                // Start navigation
                            },
                            "🧭 Start Navigation"
                        }
                        button {
                            class: "glass-button py-2 px-3 rounded text-sm",
                            onclick: move |_| {
                                // Share directions
                            },
                            "📤"
                        }
                    }
                }
            } else {
                div { class: "text-center py-6",
                    div { class: "text-4xl mb-3", "🗺️" }
                    p { class: "text-sm text-gray-600 mb-2", "No job selected" }
                    p { class: "text-xs text-gray-500", "Select a job to get directions" }
                }
            }
        }
    }
}

#[component]
fn DirectionStep(
    step: u8,
    instruction: String,
    distance: String,
    icon: String
) -> Element {
    rsx! {
        div { class: "flex items-center space-x-3 glass-control p-2 rounded",
            div { class: "flex-shrink-0 w-8 h-8 bg-blue-500 text-white rounded-full flex items-center justify-center text-xs font-bold",
                "{step}"
            }
            div { class: "flex-1 min-w-0",
                p { class: "text-sm font-medium text-gray-900 dark:text-white truncate",
                    "{instruction}"
                }
                p { class: "text-xs text-gray-500", "{distance}" }
            }
            span { class: "text-lg flex-shrink-0", "{icon}" }
        }
    }
}

// Enhanced comprehensive mock data with Australian locations (using correct Job struct)
fn get_enhanced_mock_jobs() -> Vec<Job> {
    use uuid::Uuid;
    use chrono::{Utc, Duration};
    
    vec![
        // Adelaide Hospital Jobs
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Senior Hospital Pharmacist".to_string(),
            description: "Leading clinical pharmacy role at Royal Adelaide Hospital. Oversee medication management for complex patient cases, mentor junior staff, and work closely with medical teams to optimise patient outcomes.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 85.0,
            pharmacy_name: "Royal Adelaide Hospital".to_string(),
            address: "Port Road, Adelaide SA 5000".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9205),
            longitude: Some(138.6052),
            start_date: Utc::now() + Duration::days(1),
            end_date: Utc::now() + Duration::days(365),
            start_time: "08:00".to_string(),
            end_time: "16:00".to_string(),
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: None,
            created_at: Utc::now() - Duration::hours(2),
            updated_at: Utc::now() - Duration::hours(2),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Community Pharmacist".to_string(),
            description: "Retail pharmacy position in Adelaide CBD. Great for experienced pharmacist seeking varied daily responsibilities.".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 72.0,
            pharmacy_name: "CBD Pharmacy".to_string(),
            address: "King William Street, Adelaide SA 5000".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9285),
            longitude: Some(138.6007),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(90),
            start_time: "09:00".to_string(),
            end_time: "17:30".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: None,
            created_at: Utc::now() - Duration::hours(5),
            updated_at: Utc::now() - Duration::hours(5),
            created_by: UserId(Uuid::new_v4()),
        },
    ]
}

// Simplified helper functions for the comprehensive map features

// Heatmap data generation for map visualisation
fn generate_heatmap_data(jobs: &[Job], heatmap_type: &str) -> Vec<HeatmapPoint> {
    jobs.iter()
        .filter_map(|job| {
            if let (Some(lat), Some(lng)) = (job.latitude, job.longitude) {
                let intensity = match heatmap_type {
                    "salary" => job.hourly_rate / 100.0, // Normalise salary
                    "density" => 0.8, // Fixed density value
                    "urgency" => if job.is_urgent { 1.0 } else { 0.3 },
                    _ => 0.5,
                };
                Some(HeatmapPoint {
                    lat,
                    lng,
                    intensity: intensity.min(1.0),
                    data_type: heatmap_type.to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

// Commute calculator with Australian transport modes
fn calculate_commute_info(from: (f64, f64), to: (f64, f64)) -> CommuteInfo {
    let distance = calculate_distance(from, to);
    
    // Australian urban transport estimates
    let driving_time = distance / 50.0 * 60.0; // 50 km/h average in cities
    let transit_time = distance / 25.0 * 60.0; // 25 km/h average with stops
    let walking_time = distance / 5.0 * 60.0; // 5 km/h walking speed
    
    // Select most appropriate mode based on distance
    if distance < 1.5 {
        CommuteInfo {
            travel_time: walking_time,
            distance,
            mode: "Walking".to_string(),
            cost: None,
        }
    } else if distance < 15.0 {
        CommuteInfo {
            travel_time: transit_time,
            distance,
            mode: "Public Transport".to_string(),
            cost: Some(4.50), // Average Adelaide metro ticket
        }
    } else {
        CommuteInfo {
            travel_time: driving_time,
            distance,
            mode: "Driving".to_string(),
            cost: Some(distance * 0.68), // ATO car allowance rate 2024
        }
    }
}

// Format distance for Australian users
fn format_distance(distance_km: f64) -> String {
    if distance_km < 1.0 {
        format!("{:.0}m", distance_km * 1000.0)
    } else {
        format!("{:.1}km", distance_km)
    }
}

// Format travel time for display
fn format_travel_time(minutes: f64) -> String {
    if minutes < 60.0 {
        format!("{:.0} min", minutes)
    } else {
        let hours = minutes / 60.0;
        let remaining_minutes = minutes % 60.0;
        if remaining_minutes < 5.0 {
            format!("{:.0}h", hours)
        } else {
            format!("{:.0}h {:.0}m", hours.floor(), remaining_minutes)
        }
    }
}
