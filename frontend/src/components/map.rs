use dioxus::prelude::*;
use shared::Job;

#[derive(Props, Clone, PartialEq)]
pub struct MapContainerProps {
    jobs: Vec<Job>,
}

#[component]
pub fn MapContainer(props: MapContainerProps) -> Element {
    rsx! {
        div {
            class: "w-full h-full relative bg-gray-100",
            id: "mapbox-container",
            
            // Map will be initialized here via JavaScript
            // This is a placeholder for Mapbox integration
            div {
                class: "absolute inset-0 flex items-center justify-center",
                div {
                    class: "text-center text-gray-500",
                    div {
                        class: "text-6xl mb-4",
                        "🗺️"
                    }
                    h3 {
                        class: "text-xl font-semibold mb-2",
                        "Mapbox Integration"
                    }
                    p {
                        class: "text-gray-600",
                        "Loading interactive map with {props.jobs.len()} job locations..."
                    }
                }
            }
            
            // Map controls overlay
            MapControls {}
            
            // Job markers (placeholder)
            for (i, job) in props.jobs.iter().enumerate() {
                JobMarker {
                    job: job.clone(),
                    x: (i as f32 * 50.0) % 300.0,
                    y: ((i as f32 * 30.0) % 200.0) + 100.0,
                }
            }
        }
    }
}

#[component]
fn MapControls() -> Element {
    rsx! {
        div {
            class: "absolute bottom-6 right-6 z-10 space-y-2",
            
            // Zoom controls
            div {
                class: "bg-white rounded shadow-lg",
                button {
                    class: "block w-10 h-10 flex items-center justify-center hover:bg-gray-50 border-b text-lg font-bold",
                    onclick: |_| {
                        // Zoom in functionality
                        web_sys::console::log_1(&"Zoom in clicked".into());
                    },
                    "+"
                }
                button {
                    class: "block w-10 h-10 flex items-center justify-center hover:bg-gray-50 text-lg font-bold",
                    onclick: |_| {
                        // Zoom out functionality  
                        web_sys::console::log_1(&"Zoom out clicked".into());
                    },
                    "−"
                }
            }
            
            // Recenter button
            button {
                class: "block w-10 h-10 bg-white rounded shadow-lg flex items-center justify-center hover:bg-gray-50",
                title: "Recenter map",
                onclick: |_| {
                    // Recenter functionality
                    web_sys::console::log_1(&"Recenter clicked".into());
                },
                "🎯"
            }
            
            // Map style toggle
            button {
                class: "block w-10 h-10 bg-white rounded shadow-lg flex items-center justify-center hover:bg-gray-50",
                title: "Toggle map style",
                onclick: |_| {
                    // Toggle map style
                    web_sys::console::log_1(&"Map style toggle clicked".into());
                },
                "🌍"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct JobMarkerProps {
    job: Job,
    x: f32,
    y: f32,
}

#[component]
fn JobMarker(props: JobMarkerProps) -> Element {
    let mut show_popup = use_signal(|| false);
    
    // Different colors for different job types
    let marker_color = match props.job.job_type {
        shared::JobType::Intern => "bg-blue-500",
        shared::JobType::Student => "bg-green-500",
        shared::JobType::Pharmacist => "bg-purple-500",
        shared::JobType::PharmacyAssistant => "bg-orange-500",
        shared::JobType::PharmacyTechnician => "bg-indigo-500",
    };
    
    rsx! {
        div {
            class: "absolute z-20",
            style: "left: {props.x}px; top: {props.y}px;",
            
            // Marker pin
            div {
                class: "relative",
                
                button {
                    class: "w-8 h-8 {marker_color} rounded-full flex items-center justify-center text-white font-bold shadow-lg hover:scale-110 transition-transform border-2 border-white",
                    onclick: move |_| {
                        show_popup.set(!show_popup());
                    },
                    onmouseenter: move |_| {
                        show_popup.set(true);
                    },
                    onmouseleave: move |_| {
                        show_popup.set(false);
                    },
                    "📍"
                }
                
                // Job popup
                if show_popup() {
                    div {
                        class: "absolute bottom-10 left-1/2 transform -translate-x-1/2 bg-white rounded-lg shadow-xl p-4 w-64 border z-30",
                        
                        div {
                            class: "space-y-2",
                            
                            h4 {
                                class: "font-semibold text-gray-900 text-sm",
                                "{props.job.title}"
                            }
                            
                            p {
                                class: "text-xs text-gray-600",
                                "{props.job.pharmacy_name}"
                            }
                            
                            div {
                                class: "flex justify-between items-center",
                                
                                span {
                                    class: "text-sm font-semibold text-teal-600",
                                    "${props.job.hourly_rate:.0}/hr"
                                }
                                
                                if props.job.is_urgent {
                                    span {
                                        class: "px-2 py-1 bg-red-100 text-red-600 text-xs rounded",
                                        "Urgent"
                                    }
                                }
                            }
                            
                            div {
                                class: "text-xs text-gray-500",
                                "📍 {props.job.suburb}"
                                if let Some(distance) = props.job.distance_km {
                                    " • {distance:.1}km away"
                                }
                            }
                            
                            div {
                                class: "flex space-x-2 mt-3",
                                
                                button {
                                    class: "flex-1 px-3 py-1 bg-gray-100 text-gray-700 rounded text-xs hover:bg-gray-200",
                                    onclick: move |_| {
                                        tracing::info!("View job details: {}", props.job.id);
                                    },
                                    "View"
                                }
                                
                                button {
                                    class: "flex-1 px-3 py-1 bg-teal-500 text-white rounded text-xs hover:bg-teal-600",
                                    onclick: move |_| {
                                        tracing::info!("Apply to job: {}", props.job.id);
                                    },
                                    "Apply"
                                }
                            }
                        }
                        
                        // Popup arrow
                        div {
                            class: "absolute top-full left-1/2 transform -translate-x-1/2",
                            div {
                                class: "w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-white"
                            }
                        }
                    }
                }
            }
        }
    }
}

// Function to initialize Mapbox (to be called from JavaScript)
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn initMapbox(token: &str, container_id: &str) -> js_sys::Promise;
}

pub async fn initialize_mapbox() -> Result<(), wasm_bindgen::JsValue> {
    let token = "pk.eyJ1IjoiaGVhbHRocGFnZXMiLCJhIjoiY21iN2liajJhMDh5djJxb2JtZDlndnR0MyJ9.nK2YyBNvR5F4GUOwwUlcvA";
    
    // This would initialize the actual Mapbox map
    web_sys::console::log_1(&format!("Initializing Mapbox with token: {}", token).into());
    
    Ok(())
}