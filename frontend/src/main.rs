mod components;
mod pages;
mod services;
mod hooks;
mod utils;
mod state;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use tracing::Level;

use crate::components::router::Route;

fn main() {
    // Set up console error hook for better debugging
    console_error_panic_hook::set_once();
    
    // Initialize tracing for WASM
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(Level::DEBUG)
            .build(),
    );

    tracing::info!("🚀 Starting Loco Platform Frontend v2");

    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { {include_str!("../assets/global.css")} }
        
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }
        
        Router::<Route> {}
    }
}