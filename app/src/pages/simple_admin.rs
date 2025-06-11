use leptos::*;
use leptos::prelude::*;

#[component]
pub fn SimpleAdmin() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Admin Dashboard"</h1>
            
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900">"Total Users"</h3>
                    <p class="text-3xl font-bold text-blue-600">"1,247"</p>
                    <p class="text-sm text-gray-500">"+12% from last month"</p>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900">"Active Jobs"</h3>
                    <p class="text-3xl font-bold text-green-600">"347"</p>
                    <p class="text-sm text-gray-500">"+8% from last month"</p>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900">"Applications"</h3>
                    <p class="text-3xl font-bold text-yellow-600">"2,156"</p>
                    <p class="text-sm text-gray-500">"+15% from last month"</p>
                </div>
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-lg font-semibold text-gray-900">"Revenue"</h3>
                    <p class="text-3xl font-bold text-purple-600">"$24,500"</p>
                    <p class="text-sm text-gray-500">"+22% from last month"</p>
                </div>
            </div>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">"Recent Users"</h2>
                    <div class="space-y-3">
                        <div class="flex justify-between items-center py-2 border-b border-gray-100">
                            <div>
                                <p class="font-medium text-gray-900">"Sarah Johnson"</p>
                                <p class="text-sm text-gray-500">"Pharmacist • Joined today"</p>
                            </div>
                            <span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs">"Active"</span>
                        </div>
                        <div class="flex justify-between items-center py-2 border-b border-gray-100">
                            <div>
                                <p class="font-medium text-gray-900">"Michael Chen"</p>
                                <p class="text-sm text-gray-500">"Employer • Joined 2 days ago"</p>
                            </div>
                            <span class="bg-blue-100 text-blue-800 px-2 py-1 rounded-full text-xs">"Employer"</span>
                        </div>
                        <div class="flex justify-between items-center py-2">
                            <div>
                                <p class="font-medium text-gray-900">"Emma Wilson"</p>
                                <p class="text-sm text-gray-500">"Pharmacist • Joined 1 week ago"</p>
                            </div>
                            <span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs">"Active"</span>
                        </div>
                    </div>
                </div>
                
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-semibold text-gray-900 mb-4">"System Status"</h2>
                    <div class="space-y-4">
                        <div class="flex justify-between items-center">
                            <span class="text-gray-700">"Server Status"</span>
                            <span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs">"Online"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span class="text-gray-700">"Database"</span>
                            <span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs">"Healthy"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span class="text-gray-700">"API Response Time"</span>
                            <span class="text-gray-900">"127ms"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span class="text-gray-700">"Uptime"</span>
                            <span class="text-gray-900">"99.9%"</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}