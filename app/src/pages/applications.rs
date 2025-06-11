use leptos::*;
use leptos::prelude::*;
use crate::components::ApplicationBoard;

#[component]
pub fn Applications() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <ApplicationBoard />
        </div>
    }
}