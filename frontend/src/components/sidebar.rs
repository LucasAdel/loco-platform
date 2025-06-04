use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::components::router::Route;

#[component]
pub fn Sidebar() -> Element {
    let nav = navigator();
    
    rsx! {
        div { class: "w-80 bg-white border-r border-gray-200 flex flex-col h-screen",
            
            // Header
            div { class: "p-6 border-b border-gray-200",
                div { 
                    class: "text-2xl font-bold text-blue-600 cursor-pointer",
                    onclick: move |_| { nav.push(Route::Home {}); },
                    "Lo.Co Platform" 
                }
                div { class: "mt-2 flex items-center text-sm text-gray-600",
                    span { class: "w-2 h-2 bg-green-400 rounded-full mr-2" }
                    "Australian Pharmacy Jobs"
                }
            }
            
            // Navigation
            nav { class: "flex-1 p-4",
                ul { class: "space-y-2",
                    
                    NavItem {
                        icon: "🏠",
                        label: "Home",
                        onclick: move |_| { nav.push(Route::Home {}); },
                    }
                    
                    NavItem {
                        icon: "💼",
                        label: "Jobs",
                        onclick: move |_| { nav.push(Route::Jobs {}); },
                    }
                    
                    NavItem {
                        icon: "🗺️",
                        label: "Map",
                        onclick: move |_| { nav.push(Route::Map {}); },
                    }
                    
                    NavItem {
                        icon: "💬",
                        label: "Forum",
                        onclick: move |_| { nav.push(Route::Forum {}); },
                    }
                    
                    NavItem {
                        icon: "👤",
                        label: "Profile",
                        onclick: move |_| { nav.push(Route::Profile {}); },
                    }
                    
                    NavItem {
                        icon: "🔔",
                        label: "Notifications",
                        onclick: move |_| { nav.push(Route::Notifications {}); },
                    }
                    
                    NavItem {
                        icon: "📅",
                        label: "Availability",
                        onclick: move |_| { nav.push(Route::Availability {}); },
                    }
                    
                    NavItem {
                        icon: "⚙️",
                        label: "Admin Panel",
                        onclick: move |_| { nav.push(Route::Admin {}); },
                    }
                    
                    NavItem {
                        icon: "🔗",
                        label: "Lo.Co Connect",
                        onclick: move |_| { nav.push(Route::Connect {}); },
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct NavItemProps {
    icon: String,
    label: String,
    onclick: EventHandler<MouseEvent>,
}

#[component]
fn NavItem(props: NavItemProps) -> Element {
    rsx! {
        li {
            button {
                class: "w-full flex items-center px-4 py-3 text-left rounded-l-lg transition-colors text-gray-700 hover:bg-gray-50",
                onclick: move |e| props.onclick.call(e),
                
                span { class: "mr-3 text-lg", "{props.icon}" }
                span { class: "font-medium", "{props.label}" }
            }
        }
    }
}