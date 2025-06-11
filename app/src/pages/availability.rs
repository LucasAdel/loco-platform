use leptos::*;
use leptos::prelude::*;
use crate::components::CalendarSystem;

#[component]
pub fn Availability() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 p-4">
            <CalendarSystem />
        </div>
    }
}