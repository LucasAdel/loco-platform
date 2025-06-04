use dioxus::prelude::*;
use crate::components::sidebar::Sidebar;

#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 flex",
            
            // Sidebar
            Sidebar {}
            
            // Main content area
            main {
                class: "flex-1 flex flex-col overflow-hidden",
                {children}
            }
        }
    }
}