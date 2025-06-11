use leptos::*;
use leptos::prelude::*;
use crate::components::JobCreationWizard;

#[component]
pub fn CreateJob() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <JobCreationWizard />
        </div>
    }
}