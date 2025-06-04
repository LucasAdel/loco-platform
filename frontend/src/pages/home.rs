use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::components::router::Route;

#[component]
pub fn HomePage() -> Element {
    let nav = navigator();
    
    rsx! {
        div { class: "p-8",
            
            // Welcome header
            div { class: "mb-8",
                h1 { class: "text-3xl font-bold text-gray-900 mb-4",
                    "Welcome to Lo.Co Platform"
                }
                p { class: "text-lg text-gray-600",
                    "Your professional Australian pharmacy job marketplace built with Rust and Dioxus"
                }
            }
            
            // Feature highlights
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8",
                
                FeatureCard {
                    icon: "💼".to_string(),
                    title: "Job Marketplace".to_string(),
                    description: "Find pharmacy opportunities across Australia with advanced filtering and real-time updates.".to_string()
                }
                
                FeatureCard {
                    icon: "🗺️".to_string(),
                    title: "Interactive Maps".to_string(),
                    description: "Explore jobs on an interactive map with location-based search and commute calculations.".to_string()
                }
                
                FeatureCard {
                    icon: "🚀".to_string(),
                    title: "Built with Rust".to_string(),
                    description: "Experience blazing fast performance and memory safety with our Rust-powered platform.".to_string()
                }
                
                FeatureCard {
                    icon: "📱".to_string(),
                    title: "Progressive Web App".to_string(),
                    description: "Access your opportunities anywhere with our mobile-optimized PWA experience.".to_string()
                }
                
                FeatureCard {
                    icon: "🤖".to_string(),
                    title: "AI-Powered Matching".to_string(),
                    description: "Smart job recommendations based on your skills, location, and preferences.".to_string()
                }
                
                FeatureCard {
                    icon: "🔐".to_string(),
                    title: "Secure & Private".to_string(),
                    description: "Your data is protected with enterprise-grade security and privacy controls.".to_string()
                }
            }
            
            // Quick actions
            div { class: "bg-gradient-to-r from-blue-500 to-teal-500 rounded-lg p-8 text-white",
                h2 { class: "text-2xl font-bold mb-4",
                    "Ready to get started?"
                }
                p { class: "text-blue-100 mb-6",
                    "Explore available pharmacy positions or post new opportunities for your team."
                }
                div { class: "flex space-x-4",
                    button {
                        class: "bg-white text-blue-600 px-6 py-3 rounded-lg font-semibold hover:bg-gray-100 transition-colors",
                        onclick: move |_| {
                            nav.push(Route::Jobs {});
                        },
                        "Browse Jobs"
                    }
                    button {
                        class: "bg-blue-600 text-white px-6 py-3 rounded-lg font-semibold hover:bg-blue-700 transition-colors border border-blue-400",
                        onclick: move |_| {
                            nav.push(Route::Jobs {});
                        },
                        "Post a Job"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct FeatureCardProps {
    icon: String,
    title: String,
    description: String,
}

#[component]
fn FeatureCard(props: FeatureCardProps) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow",
            
            div { class: "text-3xl mb-4",
                "{props.icon}"
            }
            
            h3 { class: "text-xl font-semibold text-gray-900 mb-2",
                "{props.title}"
            }
            
            p { class: "text-gray-600",
                "{props.description}"
            }
        }
    }
}