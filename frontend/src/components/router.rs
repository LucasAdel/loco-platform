use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::{home::HomePage, jobs::JobsPage, map::MapPage, map_simple::SimpleMapPage};
use crate::components::layout::Layout;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/jobs")]
    Jobs {},
    #[route("/map")]
    Map {},
    #[route("/forum")]
    Forum {},
    #[route("/profile")]
    Profile {},
    #[route("/notifications")]
    Notifications {},
    #[route("/availability")]
    Availability {},
    #[route("/admin")]
    Admin {},
    #[route("/connect")]
    Connect {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
fn Home() -> Element {
    rsx! { 
        Layout {
            HomePage {}
        }
    }
}

#[component]
fn Jobs() -> Element {
    rsx! { 
        Layout {
            JobsPage {}
        }
    }
}

#[component]
fn Map() -> Element {
    // Use the comprehensive MapPage with all advanced features
    rsx! { 
        MapPage {}
    }
}

#[component]
fn Forum() -> Element {
    rsx! {
        Layout {
            div { class: "p-8",
                h1 { class: "text-2xl font-bold", "Forum" }
                p { "Forum functionality coming soon..." }
            }
        }
    }
}

#[component]
fn Profile() -> Element {
    rsx! {
        Layout {
            div { class: "p-8",
                h1 { class: "text-2xl font-bold", "Profile" }
                p { "Profile management coming soon..." }
            }
        }
    }
}

#[component]
fn Notifications() -> Element {
    rsx! {
        Layout {
            div { class: "p-8",
                h1 { class: "text-2xl font-bold", "Notifications" }
                p { "Notifications coming soon..." }
            }
        }
    }
}

#[component]
fn Availability() -> Element {
    rsx! {
        Layout {
            div { class: "p-8",
                h1 { class: "text-2xl font-bold", "Availability" }
                p { "Availability management coming soon..." }
            }
        }
    }
}

#[component]
fn Admin() -> Element {
    rsx! {
        Layout {
            div { class: "p-8",
                h1 { class: "text-2xl font-bold", "Admin Panel" }
                p { "Admin functionality coming soon..." }
            }
        }
    }
}

#[component]
fn Connect() -> Element {
    rsx! {
        Layout {
            div { class: "p-8",
                h1 { class: "text-2xl font-bold", "Lo.Co Connect" }
                p { "Connect features coming soon..." }
            }
        }
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let nav = navigator();
    
    rsx! {
        Layout {
            div { class: "p-8 text-center",
                div { class: "max-w-md mx-auto",
                    h1 { class: "text-6xl font-bold text-gray-400 mb-4", "404" }
                    h2 { class: "text-2xl font-bold text-gray-800 mb-4", "Page Not Found" }
                    p { class: "text-gray-600 mb-6",
                        "The page \"/{segments.join(\"/\")}\" could not be found."
                    }
                    div { class: "space-y-4",
                        button {
                            class: "w-full bg-blue-500 text-white px-6 py-3 rounded-lg font-semibold hover:bg-blue-600 transition-colors",
                            onclick: move |_| {
                                nav.push(Route::Home {});
                            },
                            "Return Home"
                        }
                        button {
                            class: "w-full bg-gray-200 text-gray-700 px-6 py-3 rounded-lg font-semibold hover:bg-gray-300 transition-colors",
                            onclick: move |_| {
                                if let Ok(history) = web_sys::window().unwrap().history() {
                                    let _ = history.back();
                                }
                            },
                            "Go Back"
                        }
                    }
                }
            }
        }
    }
}