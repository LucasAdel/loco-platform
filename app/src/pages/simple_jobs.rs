use leptos::*;
use leptos::prelude::*;
use leptos_router::*;

#[component]
pub fn SimpleJobs() -> impl IntoView {
    let jobs = vec![
        ("Senior Pharmacist", "City Pharmacy", "Sydney, NSW", "$85,000 - $95,000"),
        ("Hospital Pharmacist", "Royal Prince Alfred", "Sydney, NSW", "$90,000 - $100,000"),
        ("Community Pharmacist", "Terry White Chemmart", "Melbourne, VIC", "$80,000 - $90,000"),
        ("Clinical Pharmacist", "Austin Health", "Melbourne, VIC", "$95,000 - $105,000"),
        ("Pharmacy Manager", "Chemist Warehouse", "Brisbane, QLD", "$100,000 - $110,000"),
    ];

    view! {
        <div class="max-w-7xl mx-auto px-4 py-8">
            <div class="flex justify-between items-center mb-8">
                <h1 class="text-3xl font-bold text-gray-900">"Jobs"</h1>
                <button class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700">
                    "Post a Job"
                </button>
            </div>
            
            <div class="mb-6">
                <input 
                    type="text" 
                    placeholder="Search jobs..." 
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                />
            </div>
            
            <div class="space-y-4">
                {jobs.into_iter().map(|(title, company, location, salary)| {
                    view! {
                        <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow">
                            <div class="flex justify-between items-start">
                                <div class="flex-1">
                                    <h3 class="text-xl font-semibold text-gray-900 mb-2">{title}</h3>
                                    <p class="text-gray-600 mb-1">{company}</p>
                                    <p class="text-gray-500 text-sm mb-3">{location}</p>
                                    <p class="text-green-600 font-semibold">{salary}</p>
                                </div>
                                <div class="flex flex-col space-y-2">
                                    <button class="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 text-sm">
                                        "Apply Now"
                                    </button>
                                    <button class="border border-gray-300 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-50 text-sm">
                                        "Save Job"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}