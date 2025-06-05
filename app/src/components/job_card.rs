use leptos::*;
use leptos_router::*;
use shared::types::SimpleJob;

#[component]
pub fn JobCard(job: SimpleJob) -> impl IntoView {
    let job_id = job.id.clone();
    
    view! {
        <A
            href=format!("/jobs/{}", job_id)
            class="block bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow p-6 border border-gray-200"
        >
            <div class="flex justify-between items-start mb-4">
                <div>
                    <h3 class="text-lg font-semibold text-gray-900">{job.title}</h3>
                    <p class="text-sm text-gray-600 mt-1">{job.company}</p>
                </div>
                <Show when=move || job.urgent>
                    <span class="bg-red-100 text-red-800 text-xs font-medium px-2.5 py-0.5 rounded">
                        "Urgent"
                    </span>
                </Show>
            </div>

            <p class="text-gray-700 text-sm mb-4 line-clamp-2">{job.description}</p>

            <div class="flex flex-wrap gap-2 mb-4">
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                    "📍 " {job.location}
                </span>
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                    "💰 " {job.salary_range}
                </span>
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-purple-100 text-purple-800">
                    {job.job_type.to_string()}
                </span>
            </div>

            <div class="flex justify-between items-center">
                <span class="text-xs text-gray-500">
                    "Posted " {job.posted_date}
                </span>
                <span class="text-blue-600 text-sm font-medium hover:text-blue-700">
                    "View Details →"
                </span>
            </div>
        </A>
    }
}