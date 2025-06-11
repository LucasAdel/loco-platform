use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::{Router, Routes, Route};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage>
                    <Route path="" view=HomePage/>
                    <Route path="dashboard" view=DashboardPage/>
                    <Route path="jobs" view=JobsPage/>
                    <Route path="map" view=MapPage/>
                    <Route path="login" view=LoginPage/>
                </Route>
                <Route path="/*any" view=NotFoundPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
            <header class="bg-white shadow-sm border-b">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        <h1 class="text-2xl font-bold text-gray-900">"Loco Platform"</h1>
                        <nav class="flex space-x-4">
                            <a href="/login" class="text-gray-600 hover:text-gray-900">"Login"</a>
                            <a href="/dashboard" class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                                "Get Started"
                            </a>
                        </nav>
                    </div>
                </div>
            </header>
            
            <section class="py-20">
                <div class="max-w-7xl mx-auto px-4 text-center">
                    <h2 class="text-5xl font-bold text-gray-900 mb-6">
                        "Find Your Dream Pharmacy Job"
                    </h2>
                    <p class="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
                        "Connect with top pharmacy employers across Australia."
                    </p>
                    <div class="flex justify-center space-x-4">
                        <a href="/jobs" class="px-8 py-3 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700">
                            "Browse Jobs"
                        </a>
                        <a href="/dashboard" class="px-8 py-3 bg-white text-blue-600 rounded-lg font-semibold border-2 border-blue-600 hover:bg-blue-50">
                            "Post a Job"
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    view! {
        <AppLayout>
            <div class="space-y-6">
                <h1 class="text-3xl font-bold text-gray-900">"Dashboard"</h1>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <div class="bg-white rounded-lg shadow p-6">
                        <h3 class="text-sm text-gray-600">"Active Jobs"</h3>
                        <p class="text-2xl font-bold text-gray-900 mt-2">"24"</p>
                    </div>
                    <div class="bg-white rounded-lg shadow p-6">
                        <h3 class="text-sm text-gray-600">"Applications"</h3>
                        <p class="text-2xl font-bold text-gray-900 mt-2">"156"</p>
                    </div>
                    <div class="bg-white rounded-lg shadow p-6">
                        <h3 class="text-sm text-gray-600">"Profile Views"</h3>
                        <p class="text-2xl font-bold text-gray-900 mt-2">"892"</p>
                    </div>
                    <div class="bg-white rounded-lg shadow p-6">
                        <h3 class="text-sm text-gray-600">"Messages"</h3>
                        <p class="text-2xl font-bold text-gray-900 mt-2">"12"</p>
                    </div>
                </div>
            </div>
        </AppLayout>
    }
}

#[component]
fn JobsPage() -> impl IntoView {
    view! {
        <AppLayout>
            <div class="space-y-6">
                <h1 class="text-3xl font-bold text-gray-900">"Job Listings"</h1>
                <div class="space-y-4">
                    <div class="bg-white rounded-lg shadow p-6">
                        <h3 class="text-xl font-semibold">"Senior Pharmacist"</h3>
                        <p class="text-gray-600">"Sydney Hospital - Sydney, NSW"</p>
                        <p class="text-gray-900 font-semibold mt-2">"$95,000 - $115,000"</p>
                    </div>
                    <div class="bg-white rounded-lg shadow p-6">
                        <h3 class="text-xl font-semibold">"Clinical Pharmacist"</h3>
                        <p class="text-gray-600">"Melbourne Medical - Melbourne, VIC"</p>
                        <p class="text-gray-900 font-semibold mt-2">"$85,000 - $105,000"</p>
                    </div>
                </div>
            </div>
        </AppLayout>
    }
}

#[component]
fn MapPage() -> impl IntoView {
    view! {
        <AppLayout>
            <div class="h-[calc(100vh-200px)]">
                <h1 class="text-3xl font-bold text-gray-900 mb-6">"Job Map"</h1>
                <div class="bg-white rounded-lg shadow h-full p-8 flex items-center justify-center">
                    <p class="text-xl text-gray-600">"Interactive map coming soon!"</p>
                </div>
            </div>
        </AppLayout>
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 flex items-center justify-center">
            <div class="bg-white rounded-lg shadow-lg p-8 w-full max-w-md">
                <h2 class="text-3xl font-bold text-gray-900 mb-8">"Welcome Back"</h2>
                <form class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Email"</label>
                        <input type="email" class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"/>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Password"</label>
                        <input type="password" class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"/>
                    </div>
                    <button type="submit" class="w-full bg-blue-600 text-white py-2 rounded-lg font-semibold hover:bg-blue-700">
                        "Sign In"
                    </button>
                </form>
                <p class="text-center text-gray-600 mt-6">
                    "Don't have an account? "
                    <a href="/register" class="text-blue-600 hover:underline">"Sign up"</a>
                </p>
            </div>
        </div>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-9xl font-bold text-gray-200">"404"</h1>
                <p class="text-2xl text-gray-600 mt-4">"Page not found"</p>
                <a href="/" class="inline-block mt-6 px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                    "Go Home"
                </a>
            </div>
        </div>
    }
}

#[component]
fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <header class="bg-white shadow-sm border-b">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        <a href="/dashboard" class="text-xl font-bold text-gray-900">"Loco Platform"</a>
                        <nav class="flex space-x-6">
                            <a href="/dashboard" class="text-gray-600 hover:text-gray-900">"Dashboard"</a>
                            <a href="/jobs" class="text-gray-600 hover:text-gray-900">"Jobs"</a>
                            <a href="/map" class="text-gray-600 hover:text-gray-900">"Map"</a>
                        </nav>
                    </div>
                </div>
            </header>
            <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                {children()}
            </main>
        </div>
    }
}