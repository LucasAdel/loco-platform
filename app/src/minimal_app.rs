use leptos::*;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    
    view! {
        <div class="min-h-screen bg-gray-100 p-8">
            <div class="max-w-4xl mx-auto">
                <h1 class="text-4xl font-bold text-gray-900 mb-8">"Loco Platform"</h1>
                
                <div class="bg-white rounded-lg shadow-lg p-8 mb-8">
                    <h2 class="text-2xl font-semibold mb-4">"Welcome to Loco Platform"</h2>
                    <p class="text-gray-600 mb-6">
                        "A modern pharmacy job platform built with Rust and Leptos."
                    </p>
                    <p class="text-sm text-gray-500">
                        "Frontend running on port 3080 | Backend API on port 3070"
                    </p>
                </div>
                
                <div class="bg-white rounded-lg shadow-lg p-8">
                    <h3 class="text-xl font-semibold mb-4">"Leptos is Working!"</h3>
                    <p class="text-gray-600 mb-4">
                        "Counter: " <span class="font-bold text-2xl text-blue-600">{count}</span>
                    </p>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                        class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                    >
                        "Click to increment"
                    </button>
                </div>
            </div>
        </div>
    }
}