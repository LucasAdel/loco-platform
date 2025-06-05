use leptos::*;
use leptos_router::*;
use crate::components::ui::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            // Hero Section
            <section class="text-center mb-16">
                <h1 class="text-5xl font-bold text-gray-900 mb-6">
                    "Find Your Next Pharmacy Role in Australia"
                </h1>
                <p class="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
                    "Connect with top pharmacy employers across Australia. Browse opportunities, 
                    explore locations, and advance your career with Loco Platform."
                </p>
                <div class="flex flex-col sm:flex-row gap-4 justify-center">
                    <A href="/jobs">
                        <Button variant=ButtonVariant::Primary size=ButtonSize::Large>
                            "Browse Jobs"
                        </Button>
                    </A>
                    <A href="/map">
                        <Button variant=ButtonVariant::Secondary size=ButtonSize::Large>
                            "Explore Map"
                        </Button>
                    </A>
                </div>
            </section>

            // Features Section
            <section class="grid md:grid-cols-3 gap-8 mb-16">
                <FeatureCard
                    icon="💼"
                    title="Latest Opportunities"
                    description="Access exclusive pharmacy positions from leading healthcare providers across Australia."
                />
                <FeatureCard
                    icon="🗺"
                    title="Interactive Map"
                    description="Visualise job locations and find opportunities in your preferred areas with our interactive map."
                />
                <FeatureCard
                    icon="🏥"
                    title="Trusted Employers"
                    description="Connect with verified pharmacy employers including hospitals, retail chains, and independent pharmacies."
                />
            </section>

            // Stats Section
            <section class="bg-blue-50 rounded-2xl p-8 mb-16">
                <div class="grid md:grid-cols-4 gap-8 text-center">
                    <StatCard number="500+" label="Active Jobs" />
                    <StatCard number="200+" label="Employers" />
                    <StatCard number="50+" label="Cities" />
                    <StatCard number="10k+" label="Registered Users" />
                </div>
            </section>

            // CTA Section
            <section class="text-center bg-gradient-to-r from-blue-600 to-blue-700 rounded-2xl p-12 text-white">
                <h2 class="text-3xl font-bold mb-4">
                    "Ready to Start Your Journey?"
                </h2>
                <p class="text-lg mb-8 opacity-90">
                    "Join thousands of pharmacy professionals finding their ideal roles."
                </p>
                <A href="/register">
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Large>
                        "Get Started Free"
                    </Button>
                </A>
            </section>
        </div>
    }
}

#[component]
fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
            <div class="text-4xl mb-4">{icon}</div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">{title}</h3>
            <p class="text-gray-600">{description}</p>
        </div>
    }
}

#[component]
fn StatCard(
    number: &'static str,
    label: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <div class="text-3xl font-bold text-blue-600">{number}</div>
            <div class="text-gray-600 mt-1">{label}</div>
        </div>
    }
}