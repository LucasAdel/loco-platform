use dioxus::prelude::*;

// Simple map page for testing routing
#[component]
pub fn SimpleMapPage() -> Element {
    rsx! {
        div { 
            class: "h-screen w-full bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center",
            
            div { class: "text-center",
                div { class: "text-6xl mb-4", "🗺️" }
                h1 { class: "text-4xl font-bold text-gray-800 mb-4", "Map Page" }
                p { class: "text-xl text-gray-600 mb-8", "Australian Pharmacy Job Locations" }
                
                div { class: "bg-white/80 backdrop-blur-xl border border-white/30 rounded-2xl p-8 shadow-xl",
                    h2 { class: "text-2xl font-semibold mb-4", "Interactive Map Coming Soon" }
                    p { class: "text-gray-600 mb-4", "This will show pharmacy job locations across Australia" }
                    
                    div { class: "grid grid-cols-2 gap-4 text-sm",
                        div { class: "flex items-center space-x-2",
                            span { "🏥" }
                            span { "Hospital Jobs" }
                        }
                        div { class: "flex items-center space-x-2",
                            span { "🏪" }
                            span { "Retail Pharmacy" }
                        }
                        div { class: "flex items-center space-x-2",
                            span { "🔬" }
                            span { "Clinical Roles" }
                        }
                        div { class: "flex items-center space-x-2",
                            span { "💼" }
                            span { "Locum Work" }
                        }
                    }
                }
                
                div { class: "mt-8",
                    button { 
                        class: "bg-blue-500 hover:bg-blue-600 text-white px-8 py-3 rounded-xl font-semibold transition-colors",
                        onclick: move |_| {
                            web_sys::console::log_1(&"Map functionality will be implemented here".into());
                        },
                        "Explore Jobs on Map"
                    }
                }
            }
        }
    }
}