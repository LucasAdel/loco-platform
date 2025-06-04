use dioxus::prelude::*;
use shared::types::Job;
use crate::components::job_list::{JobList, get_mock_jobs};
use crate::components::search_bar::SearchBar;

#[component]
pub fn JobsPage() -> Element {
    let jobs = use_signal(|| get_mock_jobs());
    let mut search_term = use_signal(|| String::new());
    
    // Filter jobs based on search term
    let filtered_jobs = jobs().into_iter()
        .filter(|job| {
            if search_term().is_empty() {
                true
            } else {
                let term = search_term().to_lowercase();
                job.title.to_lowercase().contains(&term) ||
                job.pharmacy_name.to_lowercase().contains(&term) ||
                job.suburb.to_lowercase().contains(&term) ||
                job.address.to_lowercase().contains(&term)
            }
        })
        .collect::<Vec<_>>();
    
    rsx! {
        div { class: "flex h-full",
            
            // Left side - Job list
            div { class: "w-96 border-r border-gray-200 bg-white flex flex-col",
                JobList {
                    jobs: filtered_jobs
                }
            }
            
            // Right side - Map and search
            div { class: "flex-1 relative",
                
                // Search bar overlay
                div { class: "absolute top-6 left-1/2 transform -translate-x-1/2 z-10",
                    SearchBar {
                        on_search: move |term: String| {
                            search_term.set(term);
                        }
                    }
                }
                
                // Map container
                div {
                    class: "w-full h-full bg-gray-100 flex items-center justify-center",
                    
                    // Map placeholder - will be replaced with actual Mapbox integration
                    div {
                        class: "text-center text-gray-500",
                        div {
                            class: "text-6xl mb-4",
                            "🗺️"
                        }
                        h3 {
                            class: "text-xl font-semibold mb-2",
                            "Interactive Map"
                        }
                        p {
                            "Mapbox integration coming soon..."
                        }
                        
                        // Mock map pin for Adelaide
                        div {
                            class: "mt-8 relative",
                            div {
                                class: "w-8 h-8 bg-teal-500 rounded-full flex items-center justify-center text-white font-bold shadow-lg",
                                "📍"
                            }
                            div {
                                class: "absolute top-8 left-1/2 transform -translate-x-1/2 bg-white px-2 py-1 rounded shadow text-sm",
                                "Adelaide"
                            }
                        }
                    }
                }
                
                // Map controls
                div {
                    class: "absolute bottom-6 right-6 z-10 space-y-2",
                    
                    // Zoom controls
                    div {
                        class: "bg-white rounded shadow-lg",
                        button {
                            class: "block w-10 h-10 flex items-center justify-center hover:bg-gray-50 border-b",
                            "+"
                        }
                        button {
                            class: "block w-10 h-10 flex items-center justify-center hover:bg-gray-50",
                            "−"
                        }
                    }
                    
                    // Recenter button
                    button {
                        class: "block w-10 h-10 bg-white rounded shadow-lg flex items-center justify-center hover:bg-gray-50",
                        "🎯"
                    }
                }
                
                // Map attribution
                div {
                    class: "absolute bottom-2 left-2 text-xs text-gray-500 bg-white bg-opacity-75 px-2 py-1 rounded",
                    "© Mapbox © OpenStreetMap "
                    a {
                        class: "text-blue-500 hover:underline",
                        href: "#",
                        "Improve this map"
                    }
                }
            }
        }
    }
}