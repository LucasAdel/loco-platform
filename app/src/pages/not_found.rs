use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center px-4">
            <div class="text-center">
                // Large 404 Text
                <h1 class="text-9xl font-bold text-gray-200">
                    "404"
                </h1>
                
                // Error Message
                <h2 class="text-3xl font-bold text-gray-900 mt-4">
                    "Page Not Found"
                </h2>
                
                <p class="text-lg text-gray-600 mt-4 max-w-md mx-auto">
                    "Sorry, we couldn't find the page you're looking for. It might have been moved, deleted, or maybe it never existed."
                </p>
                
                // Illustration
                <div class="my-8 text-6xl">
                    "🔍"
                </div>
                
                // Actions
                <div class="flex flex-col sm:flex-row gap-4 justify-center mt-8">
                    <A href="/">
                        <Button variant=ButtonVariant::Primary size=ButtonSize::Large>
                            "Go to Homepage"
                        </Button>
                    </A>
                    <A href="/jobs">
                        <Button variant=ButtonVariant::Secondary size=ButtonSize::Large>
                            "Browse Jobs"
                        </Button>
                    </A>
                </div>
                
                // Helpful Links
                <div class="mt-12">
                    <p class="text-sm text-gray-600 mb-4">
                        "Here are some helpful links:"
                    </p>
                    <div class="flex flex-wrap justify-center gap-4 text-sm">
                        <A href="/jobs" class="text-blue-600 hover:text-blue-800">
                            "Job Listings"
                        </A>
                        <span class="text-gray-400">"•"</span>
                        <A href="/map" class="text-blue-600 hover:text-blue-800">
                            "Job Map"
                        </A>
                        <span class="text-gray-400">"•"</span>
                        <A href="/profile" class="text-blue-600 hover:text-blue-800">
                            "My Profile"
                        </A>
                        <span class="text-gray-400">"•"</span>
                        <A href="/help" class="text-blue-600 hover:text-blue-800">
                            "Help Centre"
                        </A>
                    </div>
                </div>
                
                // Support Contact
                <div class="mt-8 text-sm text-gray-600">
                    <p>
                        "Need help? "
                        <a href="mailto:support@locoplatform.com" class="text-blue-600 hover:text-blue-800">
                            "Contact Support"
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}