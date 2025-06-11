use leptos::*;
use leptos::prelude::*;
use leptos::logging;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (api_info, set_api_info) = signal(None::<ApiInfo>);
    let (loading, set_loading) = signal(true);
    
    // Fetch API info on mount
    spawn_local(async move {
        set_loading.set(true);
        match fetch_api_info().await {
            Ok(info) => {
                set_api_info.set(Some(info));
            }
            Err(e) => {
                logging::error!("Failed to fetch API info: {}", e);
            }
        }
        set_loading.set(false);
    });
    
    view! {
        <div class="min-h-screen bg-gray-100 p-8">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-4xl font-bold text-gray-900 mb-8">"Loco Platform"</h1>
                
                <div class="bg-white rounded-lg shadow-lg p-8 mb-8">
                    <h2 class="text-2xl font-semibold mb-4">"Welcome to Loco Platform"</h2>
                    <p class="text-gray-600 mb-6">
                        "A modern pharmacy job platform built with Rust and Leptos."
                    </p>
                    
                    <div class="flex gap-4">
                        <a href="/dashboard" class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                            "Dashboard"
                        </a>
                        <a href="/jobs" class="px-6 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700">
                            "Browse Jobs"
                        </a>
                    </div>
                </div>
                
                <div class="bg-white rounded-lg shadow-lg p-8">
                    <h3 class="text-xl font-semibold mb-4">"API Connection Status"</h3>
                    {move || {
                        if loading.get() {
                            view! {
                                <div class="space-y-2">
                                    <p class="text-gray-600">"Loading API information..."</p>
                                </div>
                            }
                        } else if let Some(info) = api_info.get() {
                            view! {
                                <div class="space-y-2">
                                    <p class="text-green-600 font-semibold">"✅ Connected to Backend API"</p>
                                    <p class="text-gray-600">"API: " {info.name}</p>
                                    <p class="text-gray-600">"Version: " {info.version}</p>
                                    <p class="text-gray-600">"Description: " {info.description}</p>
                                </div>
                            }
                        } else {
                            view! {
                                <div class="space-y-2">
                                    <p class="text-red-600">"❌ Could not connect to API"</p>
                                </div>
                            }
                        }
                    }}
                </div>
                
                <div class="bg-white rounded-lg shadow-lg p-8 mt-6">
                    <h3 class="text-xl font-semibold mb-4">"Leptos Counter Demo"</h3>
                    <p class="text-gray-600 mb-4">
                        "Count: " {move || count.get()}
                    </p>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                        class="px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700"
                    >
                        "Click me!"
                    </button>
                </div>
            </div>
        </div>
    }
}

async fn fetch_api_info() -> Result<ApiInfo, String> {
    let response = Request::get("/api")
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.ok() {
        response
            .json::<ApiInfo>()
            .await
            .map_err(|e| format!("Failed to parse API info: {}", e))
    } else {
        Err(format!("API error: {}", response.status()))
    }
}