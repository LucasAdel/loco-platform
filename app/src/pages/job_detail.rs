use leptos::*;
use leptos_router::*;
use crate::api::jobs::fetch_job_by_id;
use crate::components::ui::{Button, ButtonVariant, ButtonSize, LoadingSpinner, SpinnerSize, Alert, AlertVariant};

#[component]
pub fn JobDetail() -> impl IntoView {
    let params = use_params_map();
    let job_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    
    let job_resource = create_resource(
        job_id,
        |id| async move {
            fetch_job_by_id(&id).await
        }
    );

    view! {
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <Suspense
                fallback=move || view! { 
                    <div class="flex justify-center py-12">
                        <LoadingSpinner size=SpinnerSize::Large />
                    </div>
                }
            >
                {move || {
                    match job_resource.get() {
                        Some(Ok(job)) => view! {
                            <div>
                                // Back button
                                <A href="/jobs" class="inline-flex items-center text-blue-600 hover:text-blue-700 mb-6">
                                    <svg class="w-5 h-5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                                    </svg>
                                    "Back to Jobs"
                                </A>

                                // Job header
                                <div class="bg-white rounded-lg shadow-md p-8 mb-6">
                                    <div class="flex justify-between items-start mb-6">
                                        <div>
                                            <h1 class="text-3xl font-bold text-gray-900 mb-2">{job.title}</h1>
                                            <p class="text-xl text-gray-600">{job.company}</p>
                                        </div>
                                        <Show when=move || job.urgent>
                                            <span class="bg-red-100 text-red-800 text-sm font-medium px-3 py-1 rounded">
                                                "Urgent"
                                            </span>
                                        </Show>
                                    </div>

                                    // Job details
                                    <div class="grid md:grid-cols-2 gap-6 mb-8">
                                        <div>
                                            <h3 class="text-sm font-semibold text-gray-500 uppercase mb-2">"Location"</h3>
                                            <p class="text-gray-900">{job.location}</p>
                                        </div>
                                        <div>
                                            <h3 class="text-sm font-semibold text-gray-500 uppercase mb-2">"Salary Range"</h3>
                                            <p class="text-gray-900">{job.salary_range}</p>
                                        </div>
                                        <div>
                                            <h3 class="text-sm font-semibold text-gray-500 uppercase mb-2">"Job Type"</h3>
                                            <p class="text-gray-900">{format!("{:?}", job.job_type)}</p>
                                        </div>
                                        <div>
                                            <h3 class="text-sm font-semibold text-gray-500 uppercase mb-2">"Posted"</h3>
                                            <p class="text-gray-900">{job.posted_date}</p>
                                        </div>
                                    </div>

                                    // Description
                                    <div class="mb-8">
                                        <h2 class="text-xl font-semibold text-gray-900 mb-4">"Description"</h2>
                                        <p class="text-gray-700 whitespace-pre-wrap">{job.description}</p>
                                    </div>

                                    // Apply button
                                    <div class="flex gap-4">
                                        <Button 
                                            variant=ButtonVariant::Primary 
                                            size=ButtonSize::Large
                                            on_click=Callback::new(move |_| {
                                                // TODO: Implement application logic
                                                web_sys::window()
                                                    .unwrap()
                                                    .alert_with_message("Application feature coming soon!")
                                                    .unwrap();
                                            })
                                        >
                                            "Apply Now"
                                        </Button>
                                        <Button 
                                            variant=ButtonVariant::Secondary 
                                            size=ButtonSize::Large
                                            on_click=Callback::new(move |_| {
                                                // TODO: Implement save logic
                                                web_sys::window()
                                                    .unwrap()
                                                    .alert_with_message("Save feature coming soon!")
                                                    .unwrap();
                                            })
                                        >
                                            "Save Job"
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        }.into_view(),
                        Some(Err(_)) => view! {
                            <Alert variant=AlertVariant::Error>
                                "Failed to load job details. Please try again later."
                            </Alert>
                        }.into_view(),
                        None => view! { <div></div> }.into_view(),
                    }
                }}
            </Suspense>
        </div>
    }
}