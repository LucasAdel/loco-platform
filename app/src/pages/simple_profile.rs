use leptos::*;
use leptos::prelude::*;

#[component]
pub fn SimpleProfile() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Profile"</h1>
            
            <div class="bg-white rounded-lg shadow p-6 mb-6">
                <div class="flex items-center space-x-6">
                    <div class="w-24 h-24 bg-gray-300 rounded-full flex items-center justify-center">
                        <span class="text-2xl text-gray-600">"JP"</span>
                    </div>
                    <div>
                        <h2 class="text-2xl font-bold text-gray-900">"John Pharmacist"</h2>
                        <p class="text-gray-600">"Senior Pharmacist"</p>
                        <p class="text-gray-500">"Sydney, NSW"</p>
                    </div>
                </div>
            </div>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">"Personal Information"</h3>
                    <div class="space-y-3">
                        <div>
                            <label class="block text-sm font-medium text-gray-700">"Email"</label>
                            <p class="text-gray-900">"john.pharmacist@email.com"</p>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700">"Phone"</label>
                            <p class="text-gray-900">"+61 400 123 456"</p>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700">"AHPRA Registration"</label>
                            <p class="text-gray-900">"PHA0001234567"</p>
                        </div>
                    </div>
                </div>
                
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">"Experience"</h3>
                    <div class="space-y-4">
                        <div>
                            <h4 class="font-semibold text-gray-900">"Senior Pharmacist"</h4>
                            <p class="text-gray-600">"City Pharmacy • 2020 - Present"</p>
                            <p class="text-sm text-gray-500">"Managing clinical pharmacy services and staff supervision"</p>
                        </div>
                        <div>
                            <h4 class="font-semibold text-gray-900">"Hospital Pharmacist"</h4>
                            <p class="text-gray-600">"Royal Prince Alfred Hospital • 2018 - 2020"</p>
                            <p class="text-sm text-gray-500">"Clinical pharmacy in acute care setting"</p>
                        </div>
                    </div>
                </div>
                
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">"Skills"</h3>
                    <div class="flex flex-wrap gap-2">
                        <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm">"Clinical Pharmacy"</span>
                        <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm">"Medication Review"</span>
                        <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm">"Patient Counselling"</span>
                        <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm">"Team Leadership"</span>
                        <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm">"Quality Assurance"</span>
                    </div>
                </div>
                
                <div class="bg-white rounded-lg shadow p-6">
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">"Education"</h3>
                    <div>
                        <h4 class="font-semibold text-gray-900">"Bachelor of Pharmacy"</h4>
                        <p class="text-gray-600">"University of Sydney • 2014 - 2018"</p>
                        <p class="text-sm text-gray-500">"First Class Honours"</p>
                    </div>
                </div>
            </div>
            
            <div class="mt-6 flex justify-end">
                <button class="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700">
                    "Edit Profile"
                </button>
            </div>
        </div>
    }
}