use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;
use wasm_bindgen::JsValue;

// Re-export modules
mod api;
mod auth;
mod components;
mod pages;
mod providers;
mod utils;

// Re-export commonly used items
pub use components::*;
pub use pages::*;
pub use auth::*;
pub use providers::*;

#[component]
pub fn App() -> impl IntoView {
    // Provide theme context
    provide_context(RwSignal::new(Theme::Light));
    
    view! {
        <Router>
            <providers::ThemeProvider>
                <providers::AuthProvider>
                    <Routes>
                        // Public routes
                        <Route path="/" view=pages::Home/>
                        <Route path="/login" view=auth::Login/>
                        <Route path="/register" view=auth::Register/>
                        <Route path="/forgot-password" view=auth::ForgotPassword/>
                        
                        // Protected routes with layout
                        <Route path="" view=MainLayout>
                            <Route path="/dashboard" view=pages::Dashboard/>
                            <Route path="/jobs" view=pages::Jobs/>
                            <Route path="/jobs/:id" view=pages::JobDetail/>
                            <Route path="/applications" view=pages::Applications/>
                            <Route path="/map" view=pages::Map/>
                            <Route path="/create-job" view=pages::CreateJob/>
                            <Route path="/availability" view=pages::Availability/>
                            <Route path="/team" view=pages::Team/>
                            <Route path="/profile" view=pages::Profile/>
                            <Route path="/settings" view=pages::Settings/>
                            <Route path="/admin" view=pages::Admin/>
                        </Route>
                        
                        // 404
                        <Route path="/*any" view=pages::NotFound/>
                    </Routes>
                </providers::AuthProvider>
            </providers::ThemeProvider>
        </Router>
    }
}

#[component]
fn MainLayout() -> impl IntoView {
    let show_sidebar = RwSignal::new(true);
    let is_mobile = RwSignal::new(false);
    
    // Check if mobile on mount
    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            let width = window.inner_width().unwrap_or(JsValue::from_f64(1200.0)).as_f64().unwrap_or(1200.0);
            is_mobile.set(width < 768.0);
            if width < 768.0 {
                show_sidebar.set(false);
            }
        }
    });
    
    view! {
        <div class="min-h-screen bg-gray-50">
            // Sidebar
            <components::Sidebar show_sidebar=show_sidebar is_mobile=is_mobile/>
            
            // Main content area
            <div class="transition-all duration-300"
                class:"lg:ml-64"=move || show_sidebar.get() && !is_mobile.get()
                class:"ml-0"=move || !show_sidebar.get() || is_mobile.get()
            >
                // Header
                <components::Header 
                    show_sidebar=show_sidebar 
                    on_toggle_sidebar=move |_| show_sidebar.update(|s| *s = !*s)
                />
                
                // Page content
                <main class="p-4 md:p-6 lg:p-8">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}

// Theme enum
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

#[component]
fn DashboardPage() -> impl IntoView {
    view! {
        <AppLayout>
            <div class="space-y-6">
                <h1 class="text-3xl font-bold text-white">"Dashboard"</h1>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <StatCard title="Total Jobs" value="156" icon="💼" color="from-blue-500 to-blue-600"/>
                    <StatCard title="Applications" value="23" icon="📋" color="from-green-500 to-green-600"/>
                    <StatCard title="Saved Jobs" value="45" icon="⭐" color="from-purple-500 to-purple-600"/>
                    <StatCard title="Profile Views" value="892" icon="👁️" color="from-pink-500 to-pink-600"/>
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
                <h1 class="text-3xl font-bold text-white">"Job Listings"</h1>
                <div class="grid gap-4">
                    <JobCard title="Senior Pharmacist" company="Sydney Hospital" location="Sydney, NSW" salary="$95,000 - $115,000"/>
                    <JobCard title="Clinical Pharmacist" company="Melbourne Medical" location="Melbourne, VIC" salary="$85,000 - $105,000"/>
                    <JobCard title="Community Pharmacist" company="Brisbane Pharmacy" location="Brisbane, QLD" salary="$80,000 - $95,000"/>
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
                <h1 class="text-3xl font-bold text-white mb-6">"Job Map"</h1>
                <div class="bg-white/10 backdrop-blur-xl rounded-xl border border-white/20 h-full flex items-center justify-center">
                    <p class="text-white text-xl">"Interactive map will be displayed here"</p>
                </div>
            </div>
        </AppLayout>
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 flex items-center justify-center">
            <div class="bg-white/10 backdrop-blur-xl rounded-2xl border border-white/20 p-8 w-full max-w-md">
                <h2 class="text-3xl font-bold text-white mb-6">"Login"</h2>
                <form class="space-y-4">
                    <input type="email" placeholder="Email" class="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-lg text-white placeholder-gray-400"/>
                    <input type="password" placeholder="Password" class="w-full px-4 py-3 bg-white/10 border border-white/20 rounded-lg text-white placeholder-gray-400"/>
                    <button class="w-full bg-gradient-to-r from-cyan-500 to-purple-500 text-white py-3 rounded-lg font-semibold hover:opacity-90 transition-opacity">
                        "Sign In"
                    </button>
                </form>
            </div>
        </div>
    }
}

#[component]
fn AppLayout(children: Children) -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();
    
    view! {
        <div class="flex h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
            // Sidebar
            <aside class="w-72 bg-white/5 backdrop-blur-2xl border-r border-white/10">
                <div class="p-6 border-b border-white/10">
                    <h1 class="text-xl font-bold text-white">"Loco Platform"</h1>
                </div>
                <nav class="p-4 space-y-2">
                    <NavLink href="/dashboard" label="Dashboard" icon="🏠" pathname=pathname/>
                    <NavLink href="/jobs" label="Jobs" icon="💼" pathname=pathname/>
                    <NavLink href="/map" label="Map" icon="🗺️" pathname=pathname/>
                </nav>
            </aside>
            
            // Main content
            <main class="flex-1 overflow-auto p-6">
                <div class="max-w-7xl mx-auto">
                    {children()}
                </div>
            </main>
        </div>
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str, icon: &'static str, pathname: Signal<String>) -> impl IntoView {
    let is_active = move || pathname() == href;
    
    view! {
        <a
            href=href
            class=move || {
                if is_active() {
                    "flex items-center space-x-3 px-4 py-3 rounded-xl bg-gradient-to-r from-cyan-500/20 to-purple-500/20 text-white border border-cyan-500/30"
                } else {
                    "flex items-center space-x-3 px-4 py-3 rounded-xl text-gray-300 hover:text-white hover:bg-white/10"
                }
            }
        >
            <span class="text-lg">{icon}</span>
            <span>{label}</span>
        </a>
    }
}

#[component]
fn StatCard(title: &'static str, value: &'static str, icon: &'static str, color: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white/10 backdrop-blur-xl rounded-2xl p-6 border border-white/20">
            <div class="flex items-center justify-between mb-4">
                <div class=format!("w-12 h-12 bg-gradient-to-br {} rounded-xl flex items-center justify-center", color)>
                    <span class="text-2xl">{icon}</span>
                </div>
            </div>
            <h3 class="text-3xl font-bold text-white">{value}</h3>
            <p class="text-gray-400">{title}</p>
        </div>
    }
}

#[component]
fn JobCard(title: &'static str, company: &'static str, location: &'static str, salary: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white/10 backdrop-blur-xl rounded-xl p-6 border border-white/20 hover:bg-white/20 transition-all cursor-pointer">
            <h3 class="text-xl font-semibold text-white mb-2">{title}</h3>
            <p class="text-gray-300">{company}</p>
            <div class="flex justify-between items-center mt-4">
                <span class="text-gray-400">{location}</span>
                <span class="text-cyan-400">{salary}</span>
            </div>
        </div>
    }
}