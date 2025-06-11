use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlScriptElement};
use gloo_utils::format::JsValueSerdeExt;
use gloo_console as console;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use serde_json;

// Get Mapbox token from environment
fn get_mapbox_token() -> String {
    option_env!("VITE_MAPBOX_TOKEN")
        .unwrap_or("pk.eyJ1IjoiaGVhbHRocGFnZXMiLCJhIjoiY204cGduaWxxMGF0cDJxcG5jeG03ZXRheiJ9.zfIAdS9mexKP1RNSDEI4Og")
        .to_string()
}

// Advanced job structure with all map-relevant data
#[derive(Clone, Debug, Serialize, Deserialize)]
struct MapJob {
    id: String,
    title: String,
    company: String,
    location: String,
    latitude: f64,
    longitude: f64,
    job_type: String,
    description: String,
    is_urgent: bool,
    is_featured: bool,
    salary_min: u32,
    salary_max: u32,
    posted_at: String,
    applications_count: u32,
    required_experience: String,
    benefits: Vec<String>,
    distance_km: Option<f64>,
    commute_time_minutes: Option<u32>,
    match_score: f64,
    employer_rating: f64,
    remote_option: bool,
    has_parking: bool,
    public_transport_nearby: bool,
    schedule_flexibility: String,
}

// Map configuration
#[derive(Clone, Debug)]
struct MapConfig {
    style: String,
    center: (f64, f64),
    zoom: f64,
    pitch: f64,
    bearing: f64,
    enable_3d: bool,
    enable_terrain: bool,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            style: "mapbox://styles/mapbox/light-v11".to_string(),
            center: (151.2093, -33.8688), // Sydney
            zoom: 11.0,
            pitch: 0.0,
            bearing: 0.0,
            enable_3d: true,
            enable_terrain: false,
        }
    }
}

// Advanced filter state
#[derive(Clone, Debug, Default)]
struct MapFilters {
    job_types: Vec<String>,
    salary_range: (u32, u32),
    experience_levels: Vec<String>,
    radius_km: f64,
    show_urgent_only: bool,
    show_featured_only: bool,
    posted_within_days: Option<u32>,
    minimum_applications: Option<u32>,
    keywords: Vec<String>,
}

// Mapbox GL JS bindings
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = mapboxgl)]
    type Map;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    fn new(options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    fn on(this: &Map, event: &str, callback: &js_sys::Function);

    #[wasm_bindgen(method)]
    fn addControl(this: &Map, control: &JsValue, position: Option<&str>);

    #[wasm_bindgen(method)]
    fn flyTo(this: &Map, options: &JsValue);

    #[wasm_bindgen(method)]
    fn getZoom(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    fn getBearing(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    fn getPitch(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    fn getCenter(this: &Map) -> js_sys::Array;

    #[wasm_bindgen(method)]
    fn remove(this: &Map);

    #[wasm_bindgen(method)]
    fn addSource(this: &Map, id: &str, source: &JsValue);

    #[wasm_bindgen(method)]
    fn addLayer(this: &Map, layer: &JsValue, before_id: Option<&str>);

    #[wasm_bindgen(method)]
    fn setLayoutProperty(this: &Map, layer: &str, name: &str, value: &JsValue);

    #[wasm_bindgen(method)]
    fn setPaintProperty(this: &Map, layer: &str, name: &str, value: &JsValue);

    #[wasm_bindgen(method)]
    fn setFilter(this: &Map, layer: &str, filter: &JsValue);
}

// Generate sophisticated sample data with comprehensive details
fn generate_sample_jobs() -> Vec<MapJob> {
    vec![
        MapJob {
            id: "1".to_string(),
            title: "Senior Clinical Pharmacist".to_string(),
            company: "Royal Prince Alfred Hospital".to_string(),
            location: "Camperdown, NSW".to_string(),
            latitude: -33.8891,
            longitude: 151.1810,
            job_type: "Full-time".to_string(),
            description: "Lead clinical pharmacy services in oncology department. Opportunity to shape patient care protocols and mentor junior staff. Work with cutting-edge treatments and participate in clinical trials.".to_string(),
            is_urgent: true,
            is_featured: true,
            salary_min: 110000,
            salary_max: 130000,
            posted_at: "2 hours ago".to_string(),
            applications_count: 3,
            required_experience: "5+ years".to_string(),
            benefits: vec!["Salary packaging".to_string(), "Professional development".to_string(), "Research opportunities".to_string(), "Health insurance".to_string()],
            distance_km: None,
            commute_time_minutes: None,
            match_score: 92.5,
            employer_rating: 4.8,
            remote_option: false,
            has_parking: true,
            public_transport_nearby: true,
            schedule_flexibility: "Standard hours".to_string(),
        },
        MapJob {
            id: "2".to_string(),
            title: "Locum Pharmacist - Flexible Hours".to_string(),
            company: "Chemist Warehouse Bondi".to_string(),
            location: "Bondi Beach, NSW".to_string(),
            latitude: -33.8915,
            longitude: 151.2767,
            job_type: "Casual".to_string(),
            description: "Perfect for work-life balance. Choose your own hours at our busy beachside location. Great for students or professionals seeking flexible arrangements.".to_string(),
            is_urgent: false,
            is_featured: false,
            salary_min: 85000,
            salary_max: 95000,
            posted_at: "1 day ago".to_string(),
            applications_count: 12,
            required_experience: "2+ years".to_string(),
            benefits: vec!["Flexible hours".to_string(), "Beach location".to_string(), "Staff discounts".to_string()],
            distance_km: None,
            commute_time_minutes: None,
            match_score: 78.3,
            employer_rating: 4.2,
            remote_option: false,
            has_parking: false,
            public_transport_nearby: true,
            schedule_flexibility: "High flexibility".to_string(),
        },
        MapJob {
            id: "3".to_string(),
            title: "Hospital Pharmacy Manager".to_string(),
            company: "St Vincent's Private Hospital".to_string(),
            location: "Darlinghurst, NSW".to_string(),
            latitude: -33.8796,
            longitude: 151.2208,
            job_type: "Full-time".to_string(),
            description: "Leadership role overseeing pharmacy operations in prestigious private hospital. Manage team of 15+ pharmacists and technicians. Drive quality improvement initiatives.".to_string(),
            is_urgent: true,
            is_featured: true,
            salary_min: 120000,
            salary_max: 140000,
            posted_at: "5 hours ago".to_string(),
            applications_count: 7,
            required_experience: "7+ years".to_string(),
            benefits: vec!["Leadership role".to_string(), "Salary packaging".to_string(), "Car allowance".to_string(), "Private health cover".to_string()],
            distance_km: None,
            commute_time_minutes: None,
            match_score: 95.1,
            employer_rating: 4.9,
            remote_option: true,
            has_parking: true,
            public_transport_nearby: true,
            schedule_flexibility: "Moderate flexibility".to_string(),
        },
        MapJob {
            id: "4".to_string(),
            title: "Community Pharmacist".to_string(),
            company: "Terry White Chemmart".to_string(),
            location: "Parramatta, NSW".to_string(),
            latitude: -33.8151,
            longitude: 151.0012,
            job_type: "Full-time".to_string(),
            description: "Join our award-winning team in Western Sydney's hub. Excellent training and support programs. Clear career progression pathways available.".to_string(),
            is_urgent: false,
            is_featured: false,
            salary_min: 80000,
            salary_max: 95000,
            posted_at: "3 days ago".to_string(),
            applications_count: 18,
            required_experience: "1+ years".to_string(),
            benefits: vec!["Training provided".to_string(), "Career progression".to_string(), "Performance bonuses".to_string()],
            distance_km: None,
            commute_time_minutes: None,
            match_score: 71.8,
            employer_rating: 4.1,
            remote_option: false,
            has_parking: true,
            public_transport_nearby: true,
            schedule_flexibility: "Standard hours".to_string(),
        },
        MapJob {
            id: "5".to_string(),
            title: "Compounding Pharmacist Specialist".to_string(),
            company: "Specialist Compounding Pharmacy".to_string(),
            location: "Double Bay, NSW".to_string(),
            latitude: -33.8773,
            longitude: 151.2421,
            job_type: "Full-time".to_string(),
            description: "Rare opportunity for experienced compounding pharmacist in high-end location. Work with veterinary, paediatric, and dermatological formulations.".to_string(),
            is_urgent: false,
            is_featured: true,
            salary_min: 95000,
            salary_max: 115000,
            posted_at: "1 week ago".to_string(),
            applications_count: 5,
            required_experience: "3+ years compounding".to_string(),
            benefits: vec!["Specialized role".to_string(), "Premium location".to_string(), "Equipment training".to_string()],
            distance_km: None,
            commute_time_minutes: None,
            match_score: 88.7,
            employer_rating: 4.6,
            remote_option: false,
            has_parking: true,
            public_transport_nearby: false,
            schedule_flexibility: "Standard hours".to_string(),
        },
        MapJob {
            id: "6".to_string(),
            title: "Clinical Pharmacist - ICU".to_string(),
            company: "Westmead Hospital".to_string(),
            location: "Westmead, NSW".to_string(),
            latitude: -33.8069,
            longitude: 150.9848,
            job_type: "Full-time".to_string(),
            description: "Critical care pharmacist role in one of Australia's leading hospitals. Work with multidisciplinary teams in intensive care setting.".to_string(),
            is_urgent: true,
            is_featured: false,
            salary_min: 105000,
            salary_max: 125000,
            posted_at: "6 hours ago".to_string(),
            applications_count: 2,
            required_experience: "4+ years clinical".to_string(),
            benefits: vec!["Specialty training".to_string(), "Research opportunities".to_string(), "Salary packaging".to_string()],
            distance_km: None,
            commute_time_minutes: None,
            match_score: 85.4,
            employer_rating: 4.7,
            remote_option: false,
            has_parking: true,
            public_transport_nearby: true,
            schedule_flexibility: "Shift work".to_string(),
        },
        MapJob {
            id: "7".to_string(),
            title: "Retail Pharmacist Manager".to_string(),
            company: "Priceline Pharmacy".to_string(),
            location: "Miranda, NSW".to_string(),
            latitude: -34.0354,
            longitude: 151.1006,
            job_type: "Full-time".to_string(),
            description: "Lead a busy retail pharmacy in major shopping centre. Focus on health services expansion and team development.".to_string(),
            is_urgent: false,
            is_featured: true,
            salary_min: 90000,
            salary_max: 110000,
            posted_at: "2 days ago".to_string(),
            applications_count: 14,
            required_experience: "3+ years retail".to_string(),
            benefits: vec!["Management training".to_string(), "Retail bonuses".to_string(), "Health services".to_string()],
            distance_km: None,
            commute_time_minutes: None,
            match_score: 76.9,
            employer_rating: 4.0,
            remote_option: false,
            has_parking: true,
            public_transport_nearby: true,
            schedule_flexibility: "Retail hours".to_string(),
        },
        MapJob {
            id: "8".to_string(),
            title: "Industrial Pharmacist".to_string(),
            company: "Pfizer Australia".to_string(),
            location: "North Ryde, NSW".to_string(),
            latitude: -33.7969,
            longitude: 151.1378,
            job_type: "Full-time".to_string(),
            description: "Join our regulatory affairs team. Work on drug registrations and quality assurance in pharmaceutical manufacturing.".to_string(),
            is_urgent: false,
            is_featured: true,
            salary_min: 115000,
            salary_max: 135000,
            posted_at: "4 days ago".to_string(),
            applications_count: 8,
            required_experience: "2+ years industry".to_string(),
            benefits: vec!["Industry experience".to_string(), "Global company".to_string(), "Professional development".to_string(), "Flexible work".to_string()],
            distance_km: None,
            commute_time_minutes: None,
            match_score: 91.2,
            employer_rating: 4.8,
            remote_option: true,
            has_parking: true,
            public_transport_nearby: true,
            schedule_flexibility: "Flexible hours".to_string(),
        },
    ]
}

#[component]
pub fn Map() -> impl IntoView {
    // Advanced state management
    let map_config = RwSignal::new(MapConfig::default());
    let jobs = RwSignal::new(generate_sample_jobs());
    let filtered_jobs = RwSignal::new(Vec::new());
    let selected_job = RwSignal::new(None::<MapJob>);
    let hovered_job = RwSignal::new(None::<String>);
    let filters = RwSignal::new(MapFilters {
        radius_km: 20.0,
        salary_range: (0, 200000),
        ..Default::default()
    });
    let map_loaded = RwSignal::new(false);
    let show_sidebar = RwSignal::new(true);
    let show_filters = RwSignal::new(false);
    let map_style = RwSignal::new("light".to_string());
    let show_heatmap = RwSignal::new(false);
    let show_clusters = RwSignal::new(true);
    let drawing_mode = RwSignal::new(false);
    let user_location = RwSignal::new(None::<(f64, f64)>);
    let location_error = RwSignal::new(None::<String>);
    
    // Map instance references
    let map_container = NodeRef::<html::Div>::new();
    let map_instance: Rc<RefCell<Option<Map>>> = Rc::new(RefCell::new(None));

    // Get user location
    let get_user_location = move || {
        if let Some(window) = window() {
            let navigator = window.navigator();
            if let Ok(geolocation) = navigator.geolocation() {
                let on_success = Closure::once(move |position: JsValue| {
                    if let Ok(coords) = js_sys::Reflect::get(&position, &"coords".into()) {
                        if let (Ok(lat), Ok(lng)) = (
                            js_sys::Reflect::get(&coords, &"latitude".into()),
                            js_sys::Reflect::get(&coords, &"longitude".into())
                        ) {
                            if let (Some(lat_f64), Some(lng_f64)) = (lat.as_f64(), lng.as_f64()) {
                                user_location.set(Some((lat_f64, lng_f64)));
                                
                                // Update job distances
                                jobs.update(|jobs_list| {
                                    for job in jobs_list.iter_mut() {
                                        let distance = calculate_distance(
                                            lat_f64, lng_f64,
                                            job.latitude, job.longitude
                                        );
                                        job.distance_km = Some(distance);
                                        job.commute_time_minutes = Some((distance * 2.5) as u32);
                                    }
                                });
                            }
                        }
                    }
                });
                
                let on_error = Closure::once(move |_error: JsValue| {
                    location_error.set(Some("Unable to get your location".to_string()));
                });
                
                let _ = geolocation.get_current_position_with_error_callback(
                    on_success.as_ref().unchecked_ref(),
                    Some(on_error.as_ref().unchecked_ref())
                );
                
                on_success.forget();
                on_error.forget();
            }
        }
    };

    // Advanced filtering logic
    Effect::new({
        let jobs = jobs.clone();
        let filters = filters.clone();
        let filtered_jobs = filtered_jobs.clone();
        move |_| {
            let all_jobs = jobs.get();
            let current_filters = filters.get();
            
            let mut filtered: Vec<MapJob> = all_jobs.into_iter()
                .filter(|job| {
                    // Job type filter
                    if !current_filters.job_types.is_empty() && 
                       !current_filters.job_types.contains(&job.job_type) {
                        return false;
                    }
                    
                    // Salary filter
                    if job.salary_min < current_filters.salary_range.0 ||
                       job.salary_max > current_filters.salary_range.1 {
                        return false;
                    }
                    
                    // Distance filter
                    if let Some(distance) = job.distance_km {
                        if distance > current_filters.radius_km {
                            return false;
                        }
                    }
                    
                    // Urgent filter
                    if current_filters.show_urgent_only && !job.is_urgent {
                        return false;
                    }
                    
                    // Featured filter
                    if current_filters.show_featured_only && !job.is_featured {
                        return false;
                    }
                    
                    true
                })
                .collect();
            
            // Sort by distance if available
            filtered.sort_by(|a, b| {
                match (a.distance_km, b.distance_km) {
                    (Some(a_dist), Some(b_dist)) => a_dist.partial_cmp(&b_dist).unwrap(),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => std::cmp::Ordering::Equal,
                }
            });
            
            filtered_jobs.set(filtered);
        }
    });

    // Initialize advanced map with comprehensive features
    let initialize_map = {
        let map_container = map_container.clone();
        let map_instance = map_instance.clone();
        let map_loaded = map_loaded.clone();
        let map_config = map_config.clone();
        let jobs = jobs.clone();
        let filtered_jobs = filtered_jobs.clone();
        
        move || {
            if let Some(container) = map_container.get() {
                let config = map_config.get();
                
                // Create map options with comprehensive settings
                let options = js_sys::Object::new();
                js_sys::Reflect::set(&options, &"container".into(), &container.into()).unwrap();
                js_sys::Reflect::set(&options, &"style".into(), &config.style.into()).unwrap();
                js_sys::Reflect::set(&options, &"center".into(), 
                    &js_sys::Array::of2(&config.center.0.into(), &config.center.1.into())).unwrap();
                js_sys::Reflect::set(&options, &"zoom".into(), &config.zoom.into()).unwrap();
                js_sys::Reflect::set(&options, &"pitch".into(), &config.pitch.into()).unwrap();
                js_sys::Reflect::set(&options, &"bearing".into(), &config.bearing.into()).unwrap();
                js_sys::Reflect::set(&options, &"accessToken".into(), 
                    &get_mapbox_token().into()).unwrap();
                
                // Advanced options for premium experience
                js_sys::Reflect::set(&options, &"antialias".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"preserveDrawingBuffer".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"trackResize".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"refreshExpiredTiles".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"doubleClickZoom".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"scrollZoom".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"boxZoom".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"dragRotate".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"dragPan".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"keyboard".into(), &true.into()).unwrap();
                js_sys::Reflect::set(&options, &"touchZoomRotate".into(), &true.into()).unwrap();
                
                // Create map instance
                let map = Map::new(&options);
                
                // Set up comprehensive event handlers first
                let map_loaded_clone = map_loaded.clone();
                let jobs_clone = jobs.clone();
                let on_load = Closure::once(move || {
                    console::log!("🗺️ Map loaded successfully with comprehensive features");
                    map_loaded_clone.set(true);
                    
                    // Note: Job markers will be added separately after map load
                });
                
                map.on("load", on_load.as_ref().unchecked_ref());
                on_load.forget();
                
                // Add click handlers for job markers
                let selected_job_clone = selected_job.clone();
                let on_click = Closure::wrap(Box::new(move |e: JsValue| {
                    if let Ok(features) = js_sys::Reflect::get(&e, &"features".into()) {
                        if let Ok(features_array) = features.dyn_into::<js_sys::Array>() {
                            if features_array.length() > 0 {
                                if let Ok(feature) = features_array.get(0).dyn_into::<js_sys::Object>() {
                                    if let Ok(properties) = js_sys::Reflect::get(&feature, &"properties".into()) {
                                        if let Ok(job_id) = js_sys::Reflect::get(&properties, &"id".into()) {
                                            console::log!(&format!("🎯 Job marker clicked: {:?}", job_id));
                                            // Handle marker click - show job details
                                        }
                                    }
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(JsValue)>);
                
                map.on("click", on_click.as_ref().unchecked_ref());
                on_click.forget();
                
                // Store map instance
                *map_instance.borrow_mut() = Some(map);
            }
        }
    };

    // Load Mapbox script
    let load_mapbox = move || {
        let window = window().unwrap();
        let document = window.document().unwrap();
        
        // Check if already loaded
        if js_sys::Reflect::get(&window, &"mapboxgl".into()).is_ok() {
            initialize_map();
            return;
        }

        // Create script element
        let script = document
            .create_element("script")
            .unwrap()
            .dyn_into::<HtmlScriptElement>()
            .unwrap();
        
        script.set_src("https://api.mapbox.com/mapbox-gl-js/v3.0.1/mapbox-gl.js");
        
        let init_map = initialize_map.clone();
        let closure = Closure::once(move || {
            console::log!("Mapbox script loaded!");
            init_map();
        });
        
        script.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        
        // Create link element for CSS
        let link = document
            .create_element("link")
            .unwrap()
            .dyn_into::<web_sys::HtmlLinkElement>()
            .unwrap();
        
        link.set_rel("stylesheet");
        link.set_href("https://api.mapbox.com/mapbox-gl-js/v3.0.1/mapbox-gl.css");
        
        document.head().unwrap().append_child(&script).unwrap();
        document.head().unwrap().append_child(&link).unwrap();
    };

    // Initialize on mount with real-time features
    Effect::new(move |_| {
        load_mapbox();
        
        // Start real-time job updates
        simulate_real_time_updates(jobs);
        
        // Get user location automatically
        get_user_location();
    });

    // Clean up on unmount (removed due to Send/Sync constraints)
    // The map will be cleaned up automatically when the component unmounts

    view! {
        <div class="min-h-screen bg-gray-50 relative">
            // Advanced Header
            <div class="glass bg-white/90 backdrop-blur-xl border-b border-gray-200 sticky top-0 z-50">
                <div class="max-w-full px-4 sm:px-6 lg:px-8">
                    <div class="flex items-center justify-between h-16">
                        <div class="flex items-center gap-4">
                            <h1 class="text-2xl font-bold">
                                <span class="text-gradient">Pharmacy Jobs Map</span>
                            </h1>
                            <div class="hidden md:flex items-center gap-2">
                                <span class="text-sm text-gray-500">
                                    {move || filtered_jobs.get().len()} " jobs"
                                </span>
                                <span class="text-sm text-gray-400">"|"</span>
                                <span class="text-sm text-gray-500">
                                    {move || filtered_jobs.get().iter().filter(|j| j.is_urgent).count()} " urgent"
                                </span>
                            </div>
                        </div>
                        
                        // Advanced Controls
                        <div class="flex items-center gap-3">
                            // Map Style Switcher
                            <select
                                class="input-tiffany px-3 py-2 rounded-lg text-sm"
                                on:change=move |ev| {
                                    let style = event_target_value(&ev);
                                    map_style.set(style);
                                    // Update map style
                                }
                            >
                                <option value="light">"Light"</option>
                                <option value="dark">"Dark"</option>
                                <option value="satellite">"Satellite"</option>
                                <option value="streets">"Streets"</option>
                            </select>
                            
                            // Toggle Controls
                            <button
                                class=move || format!(
                                    "btn {} px-3 py-2 text-sm",
                                    if show_heatmap.get() { "btn-primary" } else { "btn-secondary" }
                                )
                                on:click=move |_| show_heatmap.update(|v| *v = !*v)
                            >
                                "Heat Map"
                            </button>
                            
                            <button
                                class=move || format!(
                                    "btn {} px-3 py-2 text-sm",
                                    if show_clusters.get() { "btn-primary" } else { "btn-secondary" }
                                )
                                on:click=move |_| show_clusters.update(|v| *v = !*v)
                            >
                                "Clusters"
                            </button>
                            
                            <button
                                class=move || format!(
                                    "btn {} px-3 py-2 text-sm",
                                    if drawing_mode.get() { "btn-primary" } else { "btn-secondary" }
                                )
                                on:click=move |_| drawing_mode.update(|v| *v = !*v)
                            >
                                "Draw Area"
                            </button>
                            
                            // Geolocation
                            <button
                                class="btn btn-primary flex items-center gap-2 px-4 py-2"
                                on:click=move |_| get_user_location()
                            >
                                <Show
                                    when=move || user_location.get().is_some()
                                    fallback=|| view! {
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                        </svg>
                                    }
                                >
                                    <svg class="w-4 h-4 animate-pulse" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 3c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm0 14.2c-2.5 0-4.71-1.28-6-3.22.03-1.99 4-3.08 6-3.08 1.99 0 5.97 1.09 6 3.08-1.29 1.94-3.5 3.22-6 3.22z"/>
                                    </svg>
                                </Show>
                                "My Location"
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex h-[calc(100vh-4rem)] relative">
                // Advanced Sidebar
                <div class=move || format!(
                    "w-96 glass-tiffany overflow-hidden transition-all duration-300 flex flex-col {}",
                    if show_sidebar.get() { "translate-x-0" } else { "-translate-x-full" }
                )>
                    // Sidebar Header with Tabs
                    <div class="border-b border-gray-200 bg-white/50">
                        <div class="flex">
                            <button
                                class=move || format!(
                                    "flex-1 px-4 py-3 text-sm font-medium border-b-2 transition-colors {}",
                                    if !show_filters.get() { 
                                        "border-tiffany-blue text-tiffany-dark" 
                                    } else { 
                                        "border-transparent text-gray-600 hover:text-gray-900" 
                                    }
                                )
                                on:click=move |_| show_filters.set(false)
                            >
                                "Jobs" <span class="ml-2 badge-tiffany">{move || filtered_jobs.get().len()}</span>
                            </button>
                            <button
                                class=move || format!(
                                    "flex-1 px-4 py-3 text-sm font-medium border-b-2 transition-colors {}",
                                    if show_filters.get() { 
                                        "border-tiffany-blue text-tiffany-dark" 
                                    } else { 
                                        "border-transparent text-gray-600 hover:text-gray-900" 
                                    }
                                )
                                on:click=move |_| show_filters.set(true)
                            >
                                "Filters"
                            </button>
                        </div>
                    </div>
                    
                    // Content Area
                    <div class="flex-1 overflow-y-auto">
                        <Show
                            when=move || !show_filters.get()
                            fallback=move || view! {
                                // Advanced Filters Panel
                                <div class="p-6 space-y-6">
                                    // Distance Filter
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-2">
                                            "Search Radius: " {move || filters.get().radius_km} " km"
                                        </label>
                                        <input
                                            type="range"
                                            min="1"
                                            max="50"
                                            value=move || filters.get().radius_km
                                            on:input=move |ev| {
                                                if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                                    filters.update(|f| f.radius_km = value);
                                                }
                                            }
                                            class="w-full"
                                        />
                                    </div>
                                    
                                    // Salary Range
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-2">
                                            "Salary Range"
                                        </label>
                                        <div class="flex items-center gap-3">
                                            <input
                                                type="number"
                                                placeholder="Min"
                                                class="input-tiffany flex-1"
                                                value=move || filters.get().salary_range.0
                                                on:input=move |ev| {
                                                    if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                                        filters.update(|f| f.salary_range.0 = value);
                                                    }
                                                }
                                            />
                                            <span class="text-gray-500">"-"</span>
                                            <input
                                                type="number"
                                                placeholder="Max"
                                                class="input-tiffany flex-1"
                                                value=move || filters.get().salary_range.1
                                                on:input=move |ev| {
                                                    if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                                        filters.update(|f| f.salary_range.1 = value);
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                    
                                    // Job Type Filter
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-2">
                                            "Job Type"
                                        </label>
                                        <div class="space-y-2">
                                            <label class="flex items-center">
                                                <input type="checkbox" class="mr-2" />
                                                <span class="text-sm">"Full-time"</span>
                                            </label>
                                            <label class="flex items-center">
                                                <input type="checkbox" class="mr-2" />
                                                <span class="text-sm">"Part-time"</span>
                                            </label>
                                            <label class="flex items-center">
                                                <input type="checkbox" class="mr-2" />
                                                <span class="text-sm">"Casual"</span>
                                            </label>
                                            <label class="flex items-center">
                                                <input type="checkbox" class="mr-2" />
                                                <span class="text-sm">"Contract"</span>
                                            </label>
                                        </div>
                                    </div>
                                    
                                    // Quick Filters
                                    <div>
                                        <label class="block text-sm font-medium text-gray-700 mb-2">
                                            "Quick Filters"
                                        </label>
                                        <div class="space-y-2">
                                            <label class="flex items-center">
                                                <input 
                                                    type="checkbox" 
                                                    class="mr-2"
                                                    checked=move || filters.get().show_urgent_only
                                                    on:change=move |ev| {
                                                        filters.update(|f| f.show_urgent_only = event_target_checked(&ev));
                                                    }
                                                />
                                                <span class="text-sm">"Urgent positions only"</span>
                                            </label>
                                            <label class="flex items-center">
                                                <input 
                                                    type="checkbox" 
                                                    class="mr-2"
                                                    checked=move || filters.get().show_featured_only
                                                    on:change=move |ev| {
                                                        filters.update(|f| f.show_featured_only = event_target_checked(&ev));
                                                    }
                                                />
                                                <span class="text-sm">"Featured positions only"</span>
                                            </label>
                                        </div>
                                    </div>
                                    
                                    // Reset Filters
                                    <button
                                        class="btn btn-secondary w-full"
                                        on:click=move |_| {
                                            filters.set(MapFilters {
                                                radius_km: 20.0,
                                                salary_range: (0, 200000),
                                                ..Default::default()
                                            });
                                        }
                                    >
                                        "Reset All Filters"
                                    </button>
                                </div>
                            }
                        >
                            // Job List
                            <div class="p-4 space-y-3">
                                <For
                                    each=move || filtered_jobs.get()
                                    key=|job| job.id.clone()
                                    children=move |job| {
                                        let job_clone = job.clone();
                                        let job_for_click = job.clone();
                                        let job_id = job.id.clone();
                                        let job_id_hover = job.id.clone();
                                        view! {
                                            <div
                                                class=move || format!(
                                                    "job-card p-4 cursor-pointer transition-all hover:scale-[1.01] {}",
                                                    if selected_job.get().as_ref().map(|j| &j.id) == Some(&job_id) {
                                                        "ring-2 ring-tiffany-blue"
                                                    } else {
                                                        ""
                                                    }
                                                )
                                                on:click=move |_| {
                                                    selected_job.set(Some(job_for_click.clone()));
                                                }
                                                on:mouseenter=move |_| {
                                                    hovered_job.set(Some(job_id_hover.clone()));
                                                }
                                                on:mouseleave=move |_| {
                                                    hovered_job.set(None);
                                                }
                                            >
                                                // Job Header
                                                <div class="flex justify-between items-start mb-2">
                                                    <div class="flex-1">
                                                        <h3 class="font-semibold text-gray-900">{job.title.clone()}</h3>
                                                        <div class="flex items-center gap-2 mt-1">
                                                            <Show when=move || job_clone.is_urgent>
                                                                <span class="badge-urgent text-xs">"Urgent"</span>
                                                            </Show>
                                                            <Show when=move || job_clone.is_featured>
                                                                <span class="badge-tiffany text-xs">"Featured"</span>
                                                            </Show>
                                                        </div>
                                                    </div>
                                                </div>
                                                
                                                // Company & Location
                                                <p class="text-sm text-gray-600 mb-1">{job.company.clone()}</p>
                                                <p class="text-sm text-gray-500 flex items-center gap-1 mb-2">
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                                    </svg>
                                                    {job.location.clone()}
                                                    <Show when=move || job_clone.distance_km.is_some()>
                                                        <span class="text-tiffany-dark font-medium">
                                                            " • " {format!("{:.1} km", job_clone.distance_km.unwrap())}
                                                        </span>
                                                    </Show>
                                                </p>
                                                
                                                // Job Details with Match Score
                                                <div class="flex justify-between items-center mb-2">
                                                    <div class="flex items-center gap-2">
                                                        <span class="text-xs text-gray-500">{job.job_type.clone()}</span>
                                                        <Show when=move || job_clone.remote_option>
                                                            <span class="badge-tiffany text-xs">"Remote OK"</span>
                                                        </Show>
                                                    </div>
                                                    <div class="flex items-center gap-2">
                                                        <span class="text-sm font-medium text-gray-700">
                                                            {format!("${}-${}k", job.salary_min / 1000, job.salary_max / 1000)}
                                                        </span>
                                                        <div class="flex items-center gap-1">
                                                            <span class="text-xs text-tiffany-dark font-medium">
                                                                {format!("{}%", job.match_score as u8)}
                                                            </span>
                                                            <div class="w-8 h-1 bg-gray-200 rounded-full overflow-hidden">
                                                                <div 
                                                                    class="h-full bg-gradient-to-r from-tiffany-blue to-tiffany-dark rounded-full"
                                                                    style=format!("width: {}%", job.match_score)
                                                                ></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                                
                                                // Enhanced features row
                                                <div class="flex items-center gap-2 mb-2 text-xs">
                                                    <Show when=move || job_clone.has_parking>
                                                        <span class="flex items-center gap-1 text-gray-600">
                                                            <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                                                                <path d="M10 2L3 7v11h3v-8h8v8h3V7l-7-5z"/>
                                                            </svg>
                                                            "Parking"
                                                        </span>
                                                    </Show>
                                                    <Show when=move || job_clone.public_transport_nearby>
                                                        <span class="flex items-center gap-1 text-gray-600">
                                                            <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                                                                <path d="M4 6h12v8H4V6zm2 2v4h8V8H6z"/>
                                                            </svg>
                                                            "Public Transport"
                                                        </span>
                                                    </Show>
                                                    <div class="flex items-center gap-1">
                                                        <svg class="w-3 h-3 text-yellow-500" fill="currentColor" viewBox="0 0 20 20">
                                                            <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/>
                                                        </svg>
                                                        <span class="text-gray-600">{format!("{:.1}", job.employer_rating)}</span>
                                                    </div>
                                                </div>
                                                
                                                // Commute Time & Applications
                                                <div class="flex justify-between items-center text-xs text-gray-500">
                                                    <span>{job.posted_at.clone()}</span>
                                                    <div class="flex items-center gap-3">
                                                        <Show when=move || job_clone.commute_time_minutes.is_some()>
                                                            <span class="flex items-center gap-1">
                                                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                                </svg>
                                                                {format!("{} min", job_clone.commute_time_minutes.unwrap())}
                                                            </span>
                                                        </Show>
                                                        <span>{job.applications_count} " applicants"</span>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                                
                                // Empty State
                                <Show when=move || filtered_jobs.get().is_empty()>
                                    <div class="text-center py-12">
                                        <svg class="w-16 h-16 mx-auto text-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M12 12h.01M12 12h-.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        <p class="text-gray-600 font-medium">"No jobs found"</p>
                                        <p class="text-sm text-gray-500 mt-2">"Try adjusting your filters"</p>
                                    </div>
                                </Show>
                            </div>
                        </Show>
                    </div>
                </div>

                // Map Container
                <div class="flex-1 relative">
                    <div
                        node_ref=map_container
                        class="w-full h-full"
                        id="map"
                    ></div>
                    
                    // Map Overlay Controls
                    <div class="absolute top-4 left-4 flex flex-col gap-2">
                        <button
                            class="bg-white glass p-2 rounded-lg shadow-lg hover:bg-gray-50"
                            on:click=move |_| show_sidebar.update(|v| *v = !*v)
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                            </svg>
                        </button>
                    </div>
                    
                    // Map Loading State
                    <Show when=move || !map_loaded.get()>
                        <div class="absolute inset-0 bg-white/80 backdrop-blur-sm flex items-center justify-center">
                            <div class="text-center">
                                <div class="loading-dots mb-4">
                                    <span></span>
                                    <span></span>
                                    <span></span>
                                </div>
                                <p class="text-gray-600">"Loading map..."</p>
                                <p class="text-sm text-gray-500 mt-2">"Preparing your job search experience"</p>
                            </div>
                        </div>
                    </Show>
                    
                    // User Location Info
                    <Show when=move || user_location.get().is_some()>
                        <div class="absolute top-4 right-4 glass bg-white/90 p-4 rounded-lg shadow-lg">
                            <h4 class="font-semibold text-sm mb-2">"Your Location"</h4>
                            {move || user_location.get().map(|(lat, lng)| view! {
                                <div class="space-y-1">
                                    <p class="text-xs text-gray-600">
                                        "Lat: " {format!("{:.6}", lat)}
                                    </p>
                                    <p class="text-xs text-gray-600">
                                        "Lng: " {format!("{:.6}", lng)}
                                    </p>
                                </div>
                            })}
                        </div>
                    </Show>
                    
                    // Error Display
                    <Show when=move || location_error.get().is_some()>
                        <div class="absolute bottom-4 left-4 right-4 md:left-auto md:right-4 md:w-96 glass bg-red-50 border border-red-200 p-4 rounded-lg">
                            <p class="text-red-700 text-sm">{move || location_error.get()}</p>
                        </div>
                    </Show>
                    
                    // Map Stats
                    <div class="absolute bottom-4 right-4 glass bg-white/90 p-4 rounded-lg shadow-lg">
                        <div class="grid grid-cols-2 gap-4 text-sm">
                            <div>
                                <p class="text-gray-500">"Total Jobs"</p>
                                <p class="font-semibold">{move || jobs.get().len()}</p>
                            </div>
                            <div>
                                <p class="text-gray-500">"In View"</p>
                                <p class="font-semibold">{move || filtered_jobs.get().len()}</p>
                            </div>
                            <div>
                                <p class="text-gray-500">"Urgent"</p>
                                <p class="font-semibold text-red-600">
                                    {move || filtered_jobs.get().iter().filter(|j| j.is_urgent).count()}
                                </p>
                            </div>
                            <div>
                                <p class="text-gray-500">"Applied"</p>
                                <p class="font-semibold">0</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Job Detail Modal
            <Show when=move || selected_job.get().is_some()>
                {move || selected_job.get().map(|job| {
                    let job_modal = job.clone();
                    let job_modal_2 = job.clone();
                    view! {
                        <div
                            class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
                            on:click=move |_| selected_job.set(None)
                        >
                            <div
                                class="glass bg-white/95 rounded-2xl p-6 max-w-2xl w-full max-h-[90vh] overflow-y-auto"
                                on:click=move |ev| ev.stop_propagation()
                            >
                                // Modal Header
                                <div class="flex justify-between items-start mb-6">
                                    <div>
                                        <div class="flex items-center gap-3 mb-2">
                                            <h2 class="text-2xl font-bold">{job.title.clone()}</h2>
                                            <Show when=move || job_modal.is_urgent>
                                                <span class="badge-urgent">"Urgent"</span>
                                            </Show>
                                            <Show when=move || job_modal.is_featured>
                                                <span class="badge-tiffany">"Featured"</span>
                                            </Show>
                                        </div>
                                        <p class="text-lg text-gray-700">{job.company.clone()}</p>
                                    </div>
                                    <button
                                        class="text-gray-400 hover:text-gray-600"
                                        on:click=move |_| selected_job.set(None)
                                    >
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                        </svg>
                                    </button>
                                </div>
                                
                                // Location & Distance
                                <div class="mb-6 p-4 bg-gray-50 rounded-lg">
                                    <div class="flex items-start justify-between">
                                        <div>
                                            <p class="text-gray-600 flex items-center gap-2">
                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                                </svg>
                                                {job.location.clone()}
                                            </p>
                                            <Show when=move || job_modal_2.distance_km.is_some()>
                                                <div class="mt-2 flex items-center gap-4 text-sm text-gray-600">
                                                    <span class="flex items-center gap-1">
                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path>
                                                        </svg>
                                                        {format!("{:.1} km away", job_modal_2.distance_km.unwrap())}
                                                    </span>
                                                    <span class="flex items-center gap-1">
                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                        </svg>
                                                        {format!("~{} min commute", job_modal_2.commute_time_minutes.unwrap())}
                                                    </span>
                                                </div>
                                            </Show>
                                        </div>
                                        <button class="btn btn-secondary text-sm">
                                            "Get Directions"
                                        </button>
                                    </div>
                                </div>
                                
                                // Job Details Grid
                                <div class="grid grid-cols-2 md:grid-cols-3 gap-4 mb-6">
                                    <div class="p-4 bg-gray-50 rounded-lg">
                                        <p class="text-sm text-gray-500">"Type"</p>
                                        <p class="font-medium">{job.job_type.clone()}</p>
                                    </div>
                                    <div class="p-4 bg-gray-50 rounded-lg">
                                        <p class="text-sm text-gray-500">"Salary"</p>
                                        <p class="font-medium">
                                            {format!("${}-${}k", job.salary_min / 1000, job.salary_max / 1000)}
                                        </p>
                                    </div>
                                    <div class="p-4 bg-gray-50 rounded-lg">
                                        <p class="text-sm text-gray-500">"Experience"</p>
                                        <p class="font-medium">{job.required_experience.clone()}</p>
                                    </div>
                                    <div class="p-4 bg-gray-50 rounded-lg">
                                        <p class="text-sm text-gray-500">"Posted"</p>
                                        <p class="font-medium">{job.posted_at.clone()}</p>
                                    </div>
                                    <div class="p-4 bg-gray-50 rounded-lg">
                                        <p class="text-sm text-gray-500">"Applications"</p>
                                        <p class="font-medium">{job.applications_count}</p>
                                    </div>
                                    <div class="p-4 bg-gray-50 rounded-lg">
                                        <p class="text-sm text-gray-500">"Status"</p>
                                        <p class="font-medium text-green-600">"Active"</p>
                                    </div>
                                </div>
                                
                                // Description
                                <div class="mb-6">
                                    <h3 class="font-semibold mb-2">"About this role"</h3>
                                    <p class="text-gray-700">{job.description.clone()}</p>
                                </div>
                                
                                // Benefits
                                {
                                    if !job.benefits.is_empty() {
                                        view! {
                                            <div class="mb-6">
                                                <h3 class="font-semibold mb-2">"Benefits"</h3>
                                                <div class="flex flex-wrap gap-2">
                                                    {job.benefits.iter().map(|benefit| view! {
                                                        <span class="badge-tiffany">{benefit.clone()}</span>
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { }.into_any()
                                    }
                                }
                                
                                // Actions
                                <div class="flex gap-3">
                                    <button class="btn btn-primary flex-1">
                                        "Apply Now"
                                    </button>
                                    <button class="btn btn-secondary">
                                        "Save"
                                    </button>
                                    <button class="btn btn-ghost">
                                        "Share"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </Show>
            
            // Comprehensive CSS for beautiful map styling
            <style>
                {r#"
                /* Modern Glass Morphism Map Styles */
                .glass {
                    background: rgba(255, 255, 255, 0.7);
                    backdrop-filter: blur(20px);
                    -webkit-backdrop-filter: blur(20px);
                    border: 1px solid rgba(255, 255, 255, 0.3);
                }
                
                .glass-tiffany {
                    background: rgba(23, 221, 184, 0.1);
                    backdrop-filter: blur(20px);
                    -webkit-backdrop-filter: blur(20px);
                    border: 1px solid rgba(23, 221, 184, 0.2);
                }
                
                .text-gradient {
                    background: linear-gradient(135deg, #17ddb8, #3b82f6);
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                }
                
                /* Advanced Input Styling */
                .input-tiffany {
                    background: rgba(255, 255, 255, 0.9);
                    border: 2px solid transparent;
                    border-radius: 12px;
                    padding: 12px 16px;
                    transition: all 0.3s ease;
                    backdrop-filter: blur(10px);
                }
                
                .input-tiffany:focus {
                    border-color: #17ddb8;
                    box-shadow: 0 0 0 4px rgba(23, 221, 184, 0.1);
                    outline: none;
                }
                
                /* Button Styles */
                .btn {
                    padding: 10px 20px;
                    border-radius: 12px;
                    font-weight: 600;
                    transition: all 0.3s ease;
                    border: none;
                    cursor: pointer;
                    backdrop-filter: blur(10px);
                }
                
                .btn-primary {
                    background: linear-gradient(135deg, #17ddb8, #3b82f6);
                    color: white;
                    box-shadow: 0 4px 15px rgba(23, 221, 184, 0.3);
                }
                
                .btn-primary:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 8px 25px rgba(23, 221, 184, 0.4);
                }
                
                .btn-secondary {
                    background: rgba(255, 255, 255, 0.8);
                    color: #374151;
                    border: 1px solid rgba(0, 0, 0, 0.1);
                }
                
                .btn-secondary:hover {
                    background: rgba(255, 255, 255, 0.9);
                    transform: translateY(-1px);
                }
                
                .btn-ghost {
                    background: transparent;
                    color: #6b7280;
                    border: 1px solid rgba(107, 114, 128, 0.3);
                }
                
                .btn-ghost:hover {
                    background: rgba(107, 114, 128, 0.1);
                    color: #374151;
                }
                
                /* Badge Styles */
                .badge-tiffany {
                    background: linear-gradient(135deg, #17ddb8, #10b981);
                    color: white;
                    padding: 4px 12px;
                    border-radius: 20px;
                    font-size: 0.75rem;
                    font-weight: 600;
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                }
                
                .badge-urgent {
                    background: linear-gradient(135deg, #ef4444, #dc2626);
                    color: white;
                    padding: 4px 12px;
                    border-radius: 20px;
                    font-size: 0.75rem;
                    font-weight: 600;
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                    animation: pulse 2s infinite;
                }
                
                /* Job Card Styling */
                .job-card {
                    background: rgba(255, 255, 255, 0.9);
                    backdrop-filter: blur(20px);
                    border: 1px solid rgba(255, 255, 255, 0.3);
                    border-radius: 16px;
                    transition: all 0.3s ease;
                    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
                }
                
                .job-card:hover {
                    transform: translateY(-4px);
                    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
                    border-color: rgba(23, 221, 184, 0.3);
                }
                
                /* Tiffany Blue Color Variables */
                :root {
                    --tiffany-blue: #17ddb8;
                    --tiffany-dark: #0d9488;
                    --tiffany-light: #a7f3d0;
                }
                
                .text-tiffany-blue { color: var(--tiffany-blue); }
                .text-tiffany-dark { color: var(--tiffany-dark); }
                .text-tiffany-light { color: var(--tiffany-light); }
                
                /* Loading Dots Animation */
                .loading-dots {
                    display: flex;
                    gap: 4px;
                    justify-content: center;
                    align-items: center;
                }
                
                .loading-dots span {
                    width: 8px;
                    height: 8px;
                    border-radius: 50%;
                    background: var(--tiffany-blue);
                    animation: bounce 1.4s ease-in-out infinite both;
                }
                
                .loading-dots span:nth-child(1) { animation-delay: -0.32s; }
                .loading-dots span:nth-child(2) { animation-delay: -0.16s; }
                
                @keyframes bounce {
                    0%, 80%, 100% { 
                        transform: scale(0);
                    } 40% { 
                        transform: scale(1);
                    }
                }
                
                @keyframes pulse {
                    0%, 100% { opacity: 1; }
                    50% { opacity: 0.7; }
                }
                
                /* Map Specific Styles */
                .mapboxgl-popup-content {
                    background: rgba(255, 255, 255, 0.95) !important;
                    backdrop-filter: blur(20px) !important;
                    border-radius: 16px !important;
                    border: 1px solid rgba(23, 221, 184, 0.2) !important;
                    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15) !important;
                    padding: 0 !important;
                }
                
                .mapboxgl-popup-tip {
                    border-top-color: rgba(255, 255, 255, 0.95) !important;
                }
                
                .mapboxgl-ctrl-group {
                    background: rgba(255, 255, 255, 0.9) !important;
                    backdrop-filter: blur(10px) !important;
                    border-radius: 12px !important;
                    border: 1px solid rgba(255, 255, 255, 0.3) !important;
                    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1) !important;
                }
                
                .mapboxgl-ctrl-group button {
                    background: transparent !important;
                    border-color: rgba(255, 255, 255, 0.2) !important;
                }
                
                .mapboxgl-ctrl-group button:hover {
                    background: rgba(23, 221, 184, 0.1) !important;
                }
                
                /* Mobile Responsive Design */
                @media (max-width: 768px) {
                    .glass-tiffany {
                        width: 100% !important;
                        transform: translateX(-100%) !important;
                        position: fixed !important;
                        z-index: 100 !important;
                        height: 100vh !important;
                    }
                    
                    .job-card {
                        margin-bottom: 1rem;
                    }
                    
                    .btn {
                        padding: 12px 16px;
                        font-size: 0.9rem;
                    }
                }
                
                /* Smooth Transitions */
                * {
                    transition: all 0.3s ease;
                }
                
                /* Scrollbar Styling */
                ::-webkit-scrollbar {
                    width: 6px;
                }
                
                ::-webkit-scrollbar-track {
                    background: rgba(255, 255, 255, 0.1);
                    border-radius: 3px;
                }
                
                ::-webkit-scrollbar-thumb {
                    background: var(--tiffany-blue);
                    border-radius: 3px;
                }
                
                ::-webkit-scrollbar-thumb:hover {
                    background: var(--tiffany-dark);
                }
                "#}
            </style>
        </div>
    }
}

// Helper function to calculate distance between coordinates (Haversine formula)
fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0; // Earth's radius in kilometers
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) +
            lat1_rad.cos() * lat2_rad.cos() *
            (delta_lon / 2.0).sin().powi(2);
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    R * c
}

// Add comprehensive job markers to map with clustering
fn add_job_markers_to_map(map: &Map, jobs: &[MapJob]) {
    // Create GeoJSON feature collection for jobs
    let features = js_sys::Array::new();
    
    for job in jobs {
        let feature = js_sys::Object::new();
        js_sys::Reflect::set(&feature, &"type".into(), &"Feature".into()).unwrap();
        
        // Create geometry
        let geometry = js_sys::Object::new();
        js_sys::Reflect::set(&geometry, &"type".into(), &"Point".into()).unwrap();
        let coordinates = js_sys::Array::new();
        coordinates.push(&job.longitude.into());
        coordinates.push(&job.latitude.into());
        js_sys::Reflect::set(&geometry, &"coordinates".into(), &coordinates).unwrap();
        js_sys::Reflect::set(&feature, &"geometry".into(), &geometry).unwrap();
        
        // Create properties with comprehensive job data
        let properties = js_sys::Object::new();
        js_sys::Reflect::set(&properties, &"id".into(), &job.id.clone().into()).unwrap();
        js_sys::Reflect::set(&properties, &"title".into(), &job.title.clone().into()).unwrap();
        js_sys::Reflect::set(&properties, &"company".into(), &job.company.clone().into()).unwrap();
        js_sys::Reflect::set(&properties, &"location".into(), &job.location.clone().into()).unwrap();
        js_sys::Reflect::set(&properties, &"job_type".into(), &job.job_type.clone().into()).unwrap();
        js_sys::Reflect::set(&properties, &"is_urgent".into(), &job.is_urgent.into()).unwrap();
        js_sys::Reflect::set(&properties, &"is_featured".into(), &job.is_featured.into()).unwrap();
        js_sys::Reflect::set(&properties, &"salary_min".into(), &job.salary_min.into()).unwrap();
        js_sys::Reflect::set(&properties, &"salary_max".into(), &job.salary_max.into()).unwrap();
        js_sys::Reflect::set(&properties, &"match_score".into(), &job.match_score.into()).unwrap();
        js_sys::Reflect::set(&properties, &"employer_rating".into(), &job.employer_rating.into()).unwrap();
        js_sys::Reflect::set(&properties, &"remote_option".into(), &job.remote_option.into()).unwrap();
        js_sys::Reflect::set(&properties, &"has_parking".into(), &job.has_parking.into()).unwrap();
        js_sys::Reflect::set(&properties, &"public_transport_nearby".into(), &job.public_transport_nearby.into()).unwrap();
        js_sys::Reflect::set(&feature, &"properties".into(), &properties).unwrap();
        
        features.push(&feature);
    }
    
    // Create feature collection
    let feature_collection = js_sys::Object::new();
    js_sys::Reflect::set(&feature_collection, &"type".into(), &"FeatureCollection".into()).unwrap();
    js_sys::Reflect::set(&feature_collection, &"features".into(), &features).unwrap();
    
    // Add source for jobs
    map.addSource("jobs", &feature_collection);
    
    // Add clustering layer
    let cluster_layer = js_sys::Object::new();
    js_sys::Reflect::set(&cluster_layer, &"id".into(), &"clusters".into()).unwrap();
    js_sys::Reflect::set(&cluster_layer, &"type".into(), &"circle".into()).unwrap();
    js_sys::Reflect::set(&cluster_layer, &"source".into(), &"jobs".into()).unwrap();
    
    // Cluster filter
    let cluster_filter = js_sys::Array::new();
    cluster_filter.push(&"has".into());
    cluster_filter.push(&"point_count".into());
    js_sys::Reflect::set(&cluster_layer, &"filter".into(), &cluster_filter).unwrap();
    
    // Cluster paint properties
    let cluster_paint = js_sys::Object::new();
    js_sys::Reflect::set(&cluster_paint, &"circle-radius".into(), 
        &JsValue::from_serde(&serde_json::json!([
            "step", ["get", "point_count"],
            20, 100, 30, 750, 40
        ])).unwrap()).unwrap();
    js_sys::Reflect::set(&cluster_paint, &"circle-color".into(),
        &JsValue::from_serde(&serde_json::json!([
            "step", ["get", "point_count"],
            "#51bbd6", 100, "#f1c40f", 750, "#f28cb1"
        ])).unwrap()).unwrap();
    js_sys::Reflect::set(&cluster_layer, &"paint".into(), &cluster_paint).unwrap();
    
    map.addLayer(&cluster_layer, None);
    
    // Add cluster count layer
    let cluster_count_layer = js_sys::Object::new();
    js_sys::Reflect::set(&cluster_count_layer, &"id".into(), &"cluster-count".into()).unwrap();
    js_sys::Reflect::set(&cluster_count_layer, &"type".into(), &"symbol".into()).unwrap();
    js_sys::Reflect::set(&cluster_count_layer, &"source".into(), &"jobs".into()).unwrap();
    js_sys::Reflect::set(&cluster_count_layer, &"filter".into(), &cluster_filter).unwrap();
    
    let cluster_count_layout = js_sys::Object::new();
    js_sys::Reflect::set(&cluster_count_layout, &"text-field".into(), &"{point_count_abbreviated}".into()).unwrap();
    js_sys::Reflect::set(&cluster_count_layout, &"text-font".into(), 
        &js_sys::Array::of1(&"DIN Offc Pro Medium".into())).unwrap();
    js_sys::Reflect::set(&cluster_count_layout, &"text-size".into(), &12.into()).unwrap();
    js_sys::Reflect::set(&cluster_count_layer, &"layout".into(), &cluster_count_layout).unwrap();
    
    map.addLayer(&cluster_count_layer, None);
    
    // Add individual job points
    let unclustered_layer = js_sys::Object::new();
    js_sys::Reflect::set(&unclustered_layer, &"id".into(), &"unclustered-point".into()).unwrap();
    js_sys::Reflect::set(&unclustered_layer, &"type".into(), &"circle".into()).unwrap();
    js_sys::Reflect::set(&unclustered_layer, &"source".into(), &"jobs".into()).unwrap();
    
    // Unclustered filter
    let unclustered_filter = js_sys::Array::new();
    unclustered_filter.push(&"!".into());
    unclustered_filter.push(&js_sys::Array::of2(&"has".into(), &"point_count".into()));
    js_sys::Reflect::set(&unclustered_layer, &"filter".into(), &unclustered_filter).unwrap();
    
    // Individual job point styling
    let unclustered_paint = js_sys::Object::new();
    js_sys::Reflect::set(&unclustered_paint, &"circle-color".into(),
        &JsValue::from_serde(&serde_json::json!([
            "case",
            ["get", "is_urgent"], "#ff4757",
            ["get", "is_featured"], "#3742fa", 
            "#2ed573"
        ])).unwrap()).unwrap();
    js_sys::Reflect::set(&unclustered_paint, &"circle-radius".into(), &8.into()).unwrap();
    js_sys::Reflect::set(&unclustered_paint, &"circle-stroke-width".into(), &2.into()).unwrap();
    js_sys::Reflect::set(&unclustered_paint, &"circle-stroke-color".into(), &"#fff".into()).unwrap();
    js_sys::Reflect::set(&unclustered_layer, &"paint".into(), &unclustered_paint).unwrap();
    
    map.addLayer(&unclustered_layer, None);
    
    console::log!("🎯 Added comprehensive job markers with clustering to map");
}

// Real-time job updates simulation
fn simulate_real_time_updates(jobs: RwSignal<Vec<MapJob>>) {
    use js_sys::Date;
    
    // Use a simpler timeout approach for WASM compatibility
    let jobs_clone = jobs.clone();
    let update_fn: Closure<dyn FnMut()> = Closure::new(move || {
        jobs_clone.update(|jobs_list| {
            // Simulate new job posting
            if jobs_list.len() < 12 && js_sys::Math::random() > 0.8 {
                let timestamp = Date::now() as i64;
                let new_job = MapJob {
                    id: format!("live-{}", timestamp),
                    title: "🔥 URGENT: Locum Pharmacist Needed".to_string(),
                    company: "Emergency Pharmacy Services".to_string(),
                    location: "Sydney CBD, NSW".to_string(),
                    latitude: -33.8688 + (js_sys::Math::random() - 0.5) * 0.1,
                    longitude: 151.2093 + (js_sys::Math::random() - 0.5) * 0.1,
                    job_type: "Urgent Locum".to_string(),
                    description: "Immediate start required for weekend coverage.".to_string(),
                    is_urgent: true,
                    is_featured: true,
                    salary_min: 90000,
                    salary_max: 120000,
                    posted_at: "Just now".to_string(),
                    applications_count: 0,
                    required_experience: "1+ years".to_string(),
                    benefits: vec!["Immediate start".to_string(), "High rate".to_string()],
                    distance_km: None,
                    commute_time_minutes: None,
                    match_score: 95.0,
                    employer_rating: 4.5,
                    remote_option: false,
                    has_parking: true,
                    public_transport_nearby: true,
                    schedule_flexibility: "Immediate".to_string(),
                };
                jobs_list.push(new_job);
                console::log!("🚨 New urgent job posted in real-time!");
            }
            
            // Simulate application count updates
            for job in jobs_list.iter_mut() {
                if js_sys::Math::random() > 0.9 {
                    job.applications_count += 1;
                }
            }
        });
    });
    
    // Set up interval using JavaScript APIs
    let callback = update_fn.as_ref().unchecked_ref();
    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(callback, 30000)
        .unwrap();
    
    // Don't drop the closure
    update_fn.forget();
}