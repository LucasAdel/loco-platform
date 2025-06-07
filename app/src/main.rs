use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <header class="bg-white shadow">
                <div class="max-w-7xl mx-auto px-4 py-6">
                    <h1 class="text-3xl font-bold text-gray-900">"Loco Platform"</h1>
                </div>
            </header>
            <main class="max-w-7xl mx-auto px-4 py-8">
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-2xl font-semibold mb-4">"Welcome to Loco Platform"</h2>
                    <p class="text-gray-600 mb-4">
                        "Your comprehensive pharmacy job platform for Australia"
                    </p>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <Card title="Jobs" count=42 color="blue" />
                        <Card title="Applications" count=7 color="green" />
                        <Card title="Companies" count=15 color="purple" />
                    </div>
                </div>
                <ApiTest />
            </main>
        </div>
    }
}

#[component]
fn Card(title: &'static str, count: i32, color: &'static str) -> impl IntoView {
    let bg_class = format!("bg-{}-100", color);
    let text_class = format!("text-{}-800", color);
    
    view! {
        <div class=format!("p-6 rounded-lg {}", bg_class)>
            <h3 class=format!("text-lg font-medium {}", text_class)>{title}</h3>
            <p class=format!("text-3xl font-bold {}", text_class)>{count}</p>
        </div>
    }
}

#[component]
fn ApiTest() -> impl IntoView {
    let (data, set_data) = create_signal("Click to test API".to_string());
    
    let test_api = move |_| {
        spawn_local(async move {
            match gloo_net::http::Request::get("http://localhost:3070/api/health")
                .send()
                .await
            {
                Ok(resp) => {
                    match resp.text().await {
                        Ok(text) => set_data.set(format!("API Response: {}", text)),
                        Err(e) => set_data.set(format!("Error reading response: {}", e)),
                    }
                }
                Err(e) => set_data.set(format!("API Error: {}", e)),
            }
        });
    };
    
    view! {
        <div class="mt-8 bg-white rounded-lg shadow p-6">
            <h3 class="text-xl font-semibold mb-4">"API Test"</h3>
            <button 
                on:click=test_api
                class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
            >
                "Test Backend API"
            </button>
            <p class="mt-4 text-gray-600">{data}</p>
        </div>
    }
}
