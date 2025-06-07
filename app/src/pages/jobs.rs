use leptos::*;
use shared::types::SimpleJobType;
use crate::components::{JobCard, SearchBar};
use crate::components::ui::{LoadingSpinner, SpinnerSize, Alert, AlertVariant};
use crate::api::jobs::fetch_jobs;

#[component]
pub fn Jobs() -> impl IntoView {
    // State management
    let (search_query, set_search_query) = create_signal(String::new());
    let (selected_type, set_selected_type) = create_signal(None::<SimpleJobType>);
    let (show_urgent_only, set_show_urgent_only) = create_signal(false);
    
    // Fetch jobs using a resource
    let jobs_resource = create_resource(
        move || (search_query.get(), selected_type.get(), show_urgent_only.get()),
        |_| async move {
            fetch_jobs().await
        }
    );

    // Filter jobs based on search criteria
    let filtered_jobs = move || {
        jobs_resource.get()
            .unwrap_or_else(|| Ok(Vec::new()))
            .unwrap_or_else(|_| Vec::new())
            .into_iter()
            .filter(|job| {
                let matches_search = search_query.get().is_empty() || 
                    job.title.to_lowercase().contains(&search_query.get().to_lowercase()) ||
                    job.location.to_lowercase().contains(&search_query.get().to_lowercase());
                
                let matches_type = selected_type.get().is_none() || 
                    selected_type.get() == Some(job.job_type.clone());
                
                let matches_urgent = !show_urgent_only.get() || job.urgent;
                
                matches_search && matches_type && matches_urgent
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-gray-900 mb-4">
                    "Pharmacy Jobs"
                </h1>
                <p class="text-gray-600">
                    "Browse the latest pharmacy opportunities across Australia"
                </p>
            </div>

            // Search and filters
            <div class="bg-white rounded-lg shadow-md p-6 mb-8">
                <div class="space-y-4">
                    <SearchBar
                        on_search=Callback::new(move |query: String| {
                            set_search_query.set(query);
                        })
                        placeholder="Search by job title or location...".to_string()
                    />
                    
                    <div class="flex flex-wrap gap-4">
                        // Job type filter
                        <select
                            class="px-4 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_selected_type.set(if value.is_empty() {
                                    None
                                } else {
                                    Some(value.into())
                                });
                            }
                        >
                            <option value="">"All Job Types"</option>
                            <option value="Full-time">"Full-time"</option>
                            <option value="Part-time">"Part-time"</option>
                            <option value="Contract">"Contract"</option>
                            <option value="Casual">"Casual"</option>
                        </select>

                        // Urgent filter
                        <label class="flex items-center">
                            <input
                                type="checkbox"
                                class="mr-2 h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                                checked=show_urgent_only
                                on:change=move |ev| {
                                    set_show_urgent_only.set(event_target_checked(&ev));
                                }
                            />
                            <span class="text-gray-700">"Urgent positions only"</span>
                        </label>
                    </div>
                </div>
            </div>

            // Jobs list
            <Suspense
                fallback=move || view! { 
                    <div class="flex justify-center py-12">
                        <LoadingSpinner size=SpinnerSize::Large />
                    </div>
                }
            >
                {move || {
                    let jobs = filtered_jobs();
                    
                    if jobs.is_empty() {
                        view! {
                            <Alert variant=AlertVariant::Info>
                                "No jobs found matching your criteria. Try adjusting your filters."
                            </Alert>
                        }.into_view()
                    } else {
                        view! {
                            <div>
                                <div class="mb-4 text-sm text-gray-600">
                                    "Found " {jobs.len()} " jobs"
                                </div>
                                <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                                    {jobs
                                        .into_iter()
                                        .map(|job| view! { <JobCard job=job /> })
                                        .collect_view()
                                    }
                                </div>
                            </div>
                        }.into_view()
                    }
                }}
            </Suspense>
        </div>
    }
}