use leptos::*;
use leptos::prelude::*;

#[component]
pub fn SimpleDashboard() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Dashboard"</h1>
            
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900">"Total Jobs"</h3>
                    <p class="text-3xl font-bold text-blue-600">"247"</p>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900">"Applications"</h3>
                    <p class="text-3xl font-bold text-green-600">"45"</p>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900">"Interviews"</h3>
                    <p class="text-3xl font-bold text-yellow-600">"12"</p>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900">"Offers"</h3>
                    <p class="text-3xl font-bold text-purple-600">"3"</p>
                </div>
            </div>
            
            <div class="bg-white rounded-lg shadow p-6">
                <h2 class="text-xl font-semibold text-gray-900 mb-4">"Recent Activity"</h2>
                <div class="space-y-3">
                    <div class="flex items-center space-x-3">
                        <div class="w-3 h-3 bg-green-500 rounded-full"></div>
                        <span class="text-gray-700">"Application submitted for Senior Pharmacist at City Pharmacy"</span>
                    </div>
                    <div class="flex items-center space-x-3">
                        <div class="w-3 h-3 bg-blue-500 rounded-full"></div>
                        <span class="text-gray-700">"Profile viewed by 5 employers"</span>
                    </div>
                    <div class="flex items-center space-x-3">
                        <div class="w-3 h-3 bg-purple-500 rounded-full"></div>
                        <span class="text-gray-700">"New job recommendation: Hospital Pharmacist"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}