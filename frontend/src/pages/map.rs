use dioxus::prelude::*;
use shared::types::{Job, JobType, JobId, UserId, Postcode, AustralianState, JobStatus};
use chrono::Utc;
use web_sys::{console, window, HtmlScriptElement};
use wasm_bindgen::prelude::*;
use uuid::Uuid;

// MapBox configuration
const MAPBOX_TOKEN: &str = "pk.eyJ1IjoiaGVhbHRocGFnZXMiLCJhIjoiY21iN2liajJhMDh5djJxb2JtZDlndnR0MyJ9.nK2YyBNvR5F4GUOwwUlcvA";

#[component]
pub fn MapPage() -> Element {
    // State management
    let jobs = use_signal(|| get_demo_jobs());
    let mut search_query = use_signal(|| String::new());
    let mut selected_job = use_signal(|| None::<JobId>);
    
    // Initialize MapBox when component mounts
    use_effect(move || {
        spawn(async move {
            initialize_mapbox().await;
        });
    });

    rsx! {
        div { class: "h-screen flex bg-gray-50",
            
            // Left Navigation Sidebar
            aside { class: "w-60 bg-white shadow-lg border-r border-gray-200 flex flex-col",
                // Logo
                div { class: "p-6 border-b border-gray-100",
                    h1 { class: "text-xl font-bold text-blue-600", "Lo.Co Connect" }
                    div { class: "flex items-center mt-2 text-sm text-gray-500",
                        span { class: "w-2 h-2 bg-green-500 rounded-full mr-2" }
                        "Viewing as: Super ..."
                    }
                }
                
                // Navigation Menu
                nav { class: "flex-1 py-4",
                    NavigationItem { icon: "🏠", label: "Home", active: false }
                    NavigationItem { icon: "💼", label: "Jobs", active: false }
                    NavigationItem { icon: "💬", label: "Forum", active: false }
                    NavigationItem { icon: "👤", label: "Profile", active: false }
                    NavigationItem { icon: "🔔", label: "Notifications", active: false }
                    NavigationItem { icon: "📅", label: "Availability", active: false }
                    NavigationItem { icon: "🗺️", label: "Map", active: true }
                    NavigationItem { icon: "⚙️", label: "Admin Panel", active: false }
                    NavigationItem { icon: "🔗", label: "Lo.Co Connect", active: false }
                }
                
                // Bottom section
                div { class: "p-4 border-t border-gray-100",
                    div { class: "flex items-center text-sm text-gray-500",
                        span { class: "mr-2", "👤" }
                        "Super Admin View"
                    }
                }
            }
            
            // Middle Jobs Panel
            div { class: "w-96 bg-white shadow-lg border-r border-gray-200 flex flex-col",
                // Header
                div { class: "p-4 border-b border-gray-100",
                    div { class: "flex items-center",
                        span { class: "w-2 h-2 bg-blue-500 rounded-full mr-2" }
                        h2 { class: "text-lg font-semibold text-gray-900", "Available Jobs" }
                    }
                }
                
                // Jobs List
                div { class: "flex-1 overflow-y-auto p-4 space-y-4",
                    for (i, job) in jobs.read().iter().enumerate() {
                        {
                            let job_clone = job.clone();
                            rsx! {
                                JobCard {
                                    key: "{i}",
                                    job: job_clone.clone(),
                                    is_selected: selected_job.read().as_ref() == Some(&job_clone.id),
                                    on_select: move |job_id| selected_job.set(Some(job_id)),
                                    on_view_on_map: move |_| {
                                        // Focus map on job location
                                        console::log_1(&format!("View on map: {}", job_clone.id.0).into());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Right Map Section
            div { class: "flex-1 flex flex-col",
                // Search Bar
                div { class: "p-4 bg-white shadow-sm border-b border-gray-200",
                    div { class: "relative max-w-md mx-auto",
                        input {
                            r#type: "text",
                            placeholder: "Search for address, suburb or postcode...",
                            class: "w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent text-gray-900 placeholder-gray-500",
                            value: search_query.read().clone(),
                            oninput: move |e| search_query.set(e.value())
                        }
                        div { class: "absolute left-3 top-1/2 transform -translate-y-1/2",
                            span { class: "text-gray-400", "🔍" }
                        }
                    }
                }
                
                // Map Container
                div { class: "flex-1 relative",
                    // MapBox container
                    div { 
                        id: "mapbox-container",
                        class: "w-full h-full"
                    }
                    
                    // Map zoom controls
                    div { class: "absolute top-4 right-4 bg-white rounded-lg shadow-lg border border-gray-200",
                        button { 
                            class: "block w-10 h-10 text-gray-600 hover:text-gray-900 hover:bg-gray-50 flex items-center justify-center border-b border-gray-200",
                            onclick: move |_| zoom_in(),
                            "+"
                        }
                        button { 
                            class: "block w-10 h-10 text-gray-600 hover:text-gray-900 hover:bg-gray-50 flex items-center justify-center",
                            onclick: move |_| zoom_out(),
                            "−"
                        }
                    }
                    
                    // Scale indicator
                    div { class: "absolute bottom-4 left-4 bg-white px-3 py-1 rounded shadow text-xs text-gray-600",
                        "5 km"
                    }
                }
            }
        }
    }
}

#[component]
fn NavigationItem(icon: String, label: String, active: bool) -> Element {
    rsx! {
        div { 
            class: if active { 
                "flex items-center px-6 py-3 text-blue-600 bg-blue-50 border-r-2 border-blue-600" 
            } else { 
                "flex items-center px-6 py-3 text-gray-600 hover:text-gray-900 hover:bg-gray-50 cursor-pointer" 
            },
            span { class: "mr-3 text-lg", "{icon}" }
            span { class: "font-medium", "{label}" }
        }
    }
}

#[component]
fn JobCard(
    job: Job,
    is_selected: bool,
    on_select: EventHandler<JobId>,
    on_view_on_map: EventHandler<JobId>
) -> Element {
    let job_type_icon = match job.job_type {
        JobType::Pharmacist => "💊",
        JobType::PharmacyTechnician => "🔬",
        JobType::Intern => "📋",
        JobType::PharmacyAssistant => "🏪",
        JobType::Student => "🎓",
    };

    rsx! {
        div { 
            class: if is_selected { 
                "bg-blue-50 border border-blue-200 rounded-lg p-4 cursor-pointer shadow-sm" 
            } else { 
                "bg-white border border-gray-200 rounded-lg p-4 cursor-pointer hover:shadow-md transition-shadow" 
            },
            onclick: move |_| on_select.call(job.id),
            
            // Job header
            div { class: "flex items-start justify-between mb-2",
                div { class: "flex-1",
                    h3 { class: "font-semibold text-gray-900 text-sm", "{job.title}" }
                    p { class: "text-sm text-gray-600 mt-1", "{job.pharmacy_name}" }
                }
                div { class: "text-right",
                    div { class: "text-lg font-bold text-teal-600", "${job.hourly_rate:.0}/hr" }
                    if job.is_urgent {
                        span { class: "inline-block bg-red-100 text-red-800 text-xs px-2 py-1 rounded-full mt-1", "Urgent" }
                    }
                }
            }
            
            // Location info
            div { class: "flex items-center text-sm text-gray-500 mb-2",
                span { class: "mr-1", "📍" }
                "{job.address}"
            }
            
            // Date and time
            div { class: "flex items-center text-sm text-gray-500 mb-3",
                span { class: "mr-4 flex items-center",
                    span { class: "mr-1", "📅" }
                    "27/5/2025"
                }
                span { class: "flex items-center",
                    span { class: "mr-1", "🕐" }
                    "{job.start_time} - {job.end_time}"
                }
            }
            
            // Action buttons
            div { class: "flex space-x-2",
                button { 
                    class: "flex-1 bg-gray-100 text-gray-700 py-2 px-3 rounded text-sm font-medium hover:bg-gray-200 transition-colors",
                    onclick: move |e| {
                        e.stop_propagation();
                        on_view_on_map.call(job.id);
                    },
                    "View on Map"
                }
                button { 
                    class: "flex-1 bg-teal-500 text-white py-2 px-3 rounded text-sm font-medium hover:bg-teal-600 transition-colors",
                    "View Details"
                }
            }
        }
    }
}

// MapBox integration functions
async fn initialize_mapbox() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Create mapbox script element
    let script: HtmlScriptElement = document.create_element("script").unwrap().dyn_into().unwrap();
    script.set_src("https://api.mapbox.com/mapbox-gl-js/v3.0.1/mapbox-gl.js");
    
    // Create mapbox CSS link
    let link = document.create_element("link").unwrap();
    link.set_attribute("href", "https://api.mapbox.com/mapbox-gl-js/v3.0.1/mapbox-gl.css").unwrap();
    link.set_attribute("rel", "stylesheet").unwrap();
    
    // Add to head
    let head = document.head().unwrap();
    head.append_child(&link).unwrap();
    head.append_child(&script).unwrap();
    
    // Wait for script to load then initialize map
    let closure = Closure::wrap(Box::new(move || {
        initialize_map_instance();
    }) as Box<dyn FnMut()>);
    
    script.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}

fn initialize_map_instance() {
    let js_code = format!(r#"
        if (typeof mapboxgl !== 'undefined') {{
            mapboxgl.accessToken = '{}';
            
            const map = new mapboxgl.Map({{
                container: 'mapbox-container',
                style: 'mapbox://styles/mapbox/streets-v12',
                center: [138.6007, -34.9285], // Adelaide coordinates
                zoom: 11
            }});
            
            // Add navigation control
            map.addControl(new mapboxgl.NavigationControl());
            
            // Add sample markers for jobs
            const jobs = [
                {{ lng: 138.6052, lat: -34.9205, title: 'Intern Position 1', rate: '$39/hr' }},
                {{ lng: 138.6403, lat: -34.9396, title: 'Intern Position 2', rate: '$41/hr' }},
                {{ lng: 138.5954, lat: -34.9078, title: 'Pharmacist Position 3', rate: '$50/hr' }},
                {{ lng: 138.5150, lat: -34.9802, title: 'Student Position 4', rate: '$35/hr' }},
                {{ lng: 138.6007, lat: -34.9285, title: 'Student Position 5', rate: '$42/hr' }}
            ];
            
            jobs.forEach(job => {{
                // Create custom marker
                const el = document.createElement('div');
                el.className = 'custom-marker';
                el.style.backgroundColor = '#14b8a6';
                el.style.width = '20px';
                el.style.height = '20px';
                el.style.borderRadius = '50%';
                el.style.border = '3px solid white';
                el.style.boxShadow = '0 2px 4px rgba(0,0,0,0.2)';
                el.style.cursor = 'pointer';
                
                // Add marker to map
                new mapboxgl.Marker(el)
                    .setLngLat([job.lng, job.lat])
                    .setPopup(new mapboxgl.Popup({{ offset: 25 }})
                        .setHTML(`<div class="p-2"><h3 class="font-semibold">${{job.title}}</h3><p class="text-teal-600 font-bold">${{job.rate}}</p></div>`))
                    .addTo(map);
            }});
            
            // Store map reference globally for zoom controls
            window.mapInstance = map;
        }}
    "#, MAPBOX_TOKEN);
    
    // Use js_sys::eval instead of window.eval
    let _ = js_sys::eval(&js_code);
}

fn zoom_in() {
    let js_code = r#"
        if (window.mapInstance) {
            window.mapInstance.zoomIn();
        }
    "#;
    let _ = js_sys::eval(js_code);
}

fn zoom_out() {
    let js_code = r#"
        if (window.mapInstance) {
            window.mapInstance.zoomOut();
        }
    "#;
    let _ = js_sys::eval(js_code);
}

// Demo job data matching the screenshot
fn get_demo_jobs() -> Vec<Job> {
    vec![
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Intern Position 1".to_string(),
            description: "Entry level pharmacy intern position".to_string(),
            job_type: JobType::Intern,
            hourly_rate: 39.0,
            pharmacy_name: "Pharmacy A".to_string(),
            address: "1 Main St, Adelaide (16.1 km)".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9205),
            longitude: Some(138.6052),
            start_date: Utc::now(),
            end_date: Utc::now(),
            start_time: "15am".to_string(),
            end_time: "5pm".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(16.1),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Intern Position 2".to_string(),
            description: "Pharmacy intern role with great learning opportunities".to_string(),
            job_type: JobType::Intern,
            hourly_rate: 41.0,
            pharmacy_name: "Pharmacy B".to_string(),
            address: "1 Main St, Adelaide (12.4 km)".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9396),
            longitude: Some(138.6403),
            start_date: Utc::now(),
            end_date: Utc::now(),
            start_time: "13am".to_string(),
            end_time: "7pm".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(12.4),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Pharmacist Position 3".to_string(),
            description: "Senior pharmacist position with competitive rate".to_string(),
            job_type: JobType::Pharmacist,
            hourly_rate: 50.0,
            pharmacy_name: "Pharmacy C".to_string(),
            address: "1 Main St, Adelaide (7.1 km)".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9078),
            longitude: Some(138.5954),
            start_date: Utc::now(),
            end_date: Utc::now(),
            start_time: "12am".to_string(),
            end_time: "7pm".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(7.1),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Student Position 4".to_string(),
            description: "Part-time student pharmacy position".to_string(),
            job_type: JobType::Student,
            hourly_rate: 35.0,
            pharmacy_name: "Pharmacy D".to_string(),
            address: "2 Main St, Adelaide (11.3 km)".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9802),
            longitude: Some(138.5150),
            start_date: Utc::now(),
            end_date: Utc::now(),
            start_time: "16am".to_string(),
            end_time: "7pm".to_string(),
            status: JobStatus::Active,
            is_urgent: false,
            distance_km: Some(11.3),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId(Uuid::new_v4()),
        },
        Job {
            id: JobId(Uuid::new_v4()),
            title: "Student Position 5".to_string(),
            description: "Urgent student position available immediately".to_string(),
            job_type: JobType::Student,
            hourly_rate: 42.0,
            pharmacy_name: "Pharmacy E".to_string(),
            address: "1 Main St, Adelaide (10.9 km)".to_string(),
            suburb: "Adelaide".to_string(),
            postcode: Postcode("5000".to_string()),
            state: AustralianState::SouthAustralia,
            latitude: Some(-34.9285),
            longitude: Some(138.6007),
            start_date: Utc::now(),
            end_date: Utc::now(),
            start_time: "16am".to_string(),
            end_time: "5pm".to_string(),
            status: JobStatus::Active,
            is_urgent: true,
            distance_km: Some(10.9),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: UserId(Uuid::new_v4()),
        },
    ]
}