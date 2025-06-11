use leptos::*;
use leptos::prelude::*;

#[component]
pub fn SimpleMap() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Job Map"</h1>
            
            <div class="bg-white rounded-lg shadow mb-6 p-4">
                <div class="flex flex-wrap gap-4 items-center">
                    <input 
                        type="text" 
                        placeholder="Search location..." 
                        class="flex-1 min-w-64 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                    />
                    <select class="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                        <option>"All Job Types"</option>
                        <option>"Pharmacist"</option>
                        <option>"Technician"</option>
                        <option>"Manager"</option>
                    </select>
                    <select class="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                        <option>"All Locations"</option>
                        <option>"Sydney"</option>
                        <option>"Melbourne"</option>
                        <option>"Brisbane"</option>
                    </select>
                </div>
            </div>
            
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2">
                    <div class="bg-gray-200 rounded-lg h-96 flex items-center justify-center">
                        <div class="text-center">
                            <svg class="mx-auto h-12 w-12 text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                            </svg>
                            <p class="text-gray-500">"Interactive map will be displayed here"</p>
                            <p class="text-sm text-gray-400">"Showing job locations across Australia"</p>
                        </div>
                    </div>
                </div>
                
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">"Nearby Jobs"</h3>
                    <div class="space-y-4">
                        <div class="border border-gray-200 rounded-lg p-4">
                            <h4 class="font-semibold text-gray-900">"Senior Pharmacist"</h4>
                            <p class="text-sm text-gray-600">"City Pharmacy"</p>
                            <p class="text-sm text-gray-500">"2.3 km away"</p>
                            <div class="mt-2">
                                <span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs">"$85K - $95K"</span>
                            </div>
                        </div>
                        
                        <div class="border border-gray-200 rounded-lg p-4">
                            <h4 class="font-semibold text-gray-900">"Hospital Pharmacist"</h4>
                            <p class="text-sm text-gray-600">"Royal Prince Alfred"</p>
                            <p class="text-sm text-gray-500">"5.1 km away"</p>
                            <div class="mt-2">
                                <span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs">"$90K - $100K"</span>
                            </div>
                        </div>
                        
                        <div class="border border-gray-200 rounded-lg p-4">
                            <h4 class="font-semibold text-gray-900">"Pharmacy Manager"</h4>
                            <p class="text-sm text-gray-600">"Chemist Warehouse"</p>
                            <p class="text-sm text-gray-500">"7.8 km away"</p>
                            <div class="mt-2">
                                <span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs">"$100K - $110K"</span>
                            </div>
                        </div>
                    </div>
                    
                    <button class="w-full mt-4 bg-blue-600 text-white py-2 rounded-lg hover:bg-blue-700">
                        "View All Jobs"
                    </button>
                </div>
            </div>
        </div>
    }
}