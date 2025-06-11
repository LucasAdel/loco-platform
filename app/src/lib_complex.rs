use leptos::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;

pub mod api;
pub mod components;
pub mod pages;

use crate::components::MapboxScripts;
use crate::api::supabase::provide_auth_context;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_auth_context();
    
    // Global state for sidebar collapse
    let sidebar_collapsed = RwSignal::new(false);
    let mobile_menu_open = RwSignal::new(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/loco-app.css"/>
        <Title text="Loco Platform - Australian Pharmacy Jobs"/>
        
        // Include Mapbox scripts
        <MapboxScripts/>
        
        <Router>
            <div class="app-container">
                <MobileMenuButton 
                    mobile_menu_open=mobile_menu_open
                />
                <MobileOverlay 
                    mobile_menu_open=mobile_menu_open
                />
                <Sidebar 
                    collapsed=sidebar_collapsed
                    mobile_menu_open=mobile_menu_open
                />
                <div class="main-content" class:sidebar-collapsed=move || sidebar_collapsed.get()>
                    <Routes fallback=|| view! { <NotFound/> }>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/dashboard") view=DashboardPage/>
                        <Route path=path!("/jobs") view=JobsPage/>
                        <Route path=path!("/map") view=MapPage/>
                        <Route path=path!("/profile") view=ProfilePage/>
                        <Route path=path!("/admin") view=AdminPage/>
                        <Route path=path!("/create-job") view=CreateJobPage/>
                        <Route path=path!("/login") view=LoginPage/>
                        <Route path=path!("/register") view=RegisterPage/>
                        <Route path=path!("/health") view=HealthPage/>
                    </Routes>
                </div>
            </div>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center">
            <div class="text-center max-w-4xl px-6">
                <h1 class="text-6xl font-bold text-gray-900 mb-6">
                    "🏥 Loco Platform"
                </h1>
                <p class="text-2xl text-gray-600 mb-8">
                    "Australian Pharmacy Jobs Platform - Built with Rust & Leptos"
                </p>
                <div class="flex justify-center space-x-6">
                    <a href="/jobs" class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 px-8 rounded-xl text-lg transition duration-200">
                        "Browse Jobs"
                    </a>
                    <a href="/health" class="bg-green-600 hover:bg-green-700 text-white font-bold py-4 px-8 rounded-xl text-lg transition duration-200">
                        "Health Check"
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn JobsPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 p-8">
            <div class="max-w-6xl mx-auto">
                <h1 class="text-4xl font-bold text-gray-900 mb-8">"Australian Pharmacy Jobs"</h1>
                
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <JobCard 
                        title="Senior Pharmacist - North Adelaide".to_string()
                        pharmacy="Women's and Children's Hospital".to_string()
                        rate="$56.00/hr".to_string()
                        location="North Adelaide, SA".to_string()
                        urgent=false
                    />
                    <JobCard 
                        title="Pharmacy Assistant - Marion".to_string()
                        pharmacy="Westfield Medical Pharmacy".to_string() 
                        rate="$28.50/hr".to_string()
                        location="Marion, SA".to_string()
                        urgent=true
                    />
                    <JobCard 
                        title="Locum Pharmacist - Glenelg".to_string()
                        pharmacy="Glenelg Beach Pharmacy".to_string()
                        rate="$65.00/hr".to_string()
                        location="Glenelg, SA".to_string()
                        urgent=true
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn JobCard(
    title: String,
    pharmacy: String,
    rate: String,
    location: String,
    urgent: bool,
) -> impl IntoView {
    let urgent_badge = if urgent {
        Some(view! {
            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800">
                "Urgent"
            </span>
        })
    } else {
        None
    };

    view! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-md transition duration-200">
            <div class="flex justify-between items-start mb-3">
                <h3 class="text-lg font-semibold text-gray-900 leading-tight">{title}</h3>
                {urgent_badge}
            </div>
            <p class="text-gray-600 mb-2">{pharmacy}</p>
            <div class="flex justify-between items-center text-sm text-gray-500">
                <span>{location}</span>
                <span class="font-semibold text-green-600">{rate}</span>
            </div>
            <button class="mt-4 w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-lg transition duration-200">
                "Apply Now"
            </button>
        </div>
    }
}

#[component]
fn HealthPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-green-50 flex items-center justify-center">
            <div class="bg-white rounded-xl shadow-lg p-8 max-w-md">
                <h1 class="text-3xl font-bold text-green-900 mb-6 text-center">"System Health"</h1>
                <div class="space-y-4">
                    <div class="flex items-center space-x-3">
                        <div class="w-4 h-4 bg-green-500 rounded-full"></div>
                        <span class="text-lg font-medium">"Frontend: Operational"</span>
                    </div>
                    <div class="flex items-center space-x-3">
                        <div class="w-4 h-4 bg-green-500 rounded-full"></div>
                        <span class="text-lg font-medium">"Backend: Connected"</span>
                    </div>
                    <div class="flex items-center space-x-3">
                        <div class="w-4 h-4 bg-green-500 rounded-full"></div>
                        <span class="text-lg font-medium">"Database: Active"</span>
                    </div>
                </div>
                <div class="mt-6 text-center">
                    <a href="/" class="text-blue-600 hover:text-blue-800 font-medium">"← Back to Home"</a>
                </div>
            </div>
        </div>
    }
}

// Mobile Menu Button Component
#[component]
fn MobileMenuButton(mobile_menu_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <button 
            class="mobile-menu-btn"
            on:click=move |_| mobile_menu_open.update(|v| *v = !*v)
        >
            <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
            </svg>
        </button>
    }
}

// Mobile Overlay Component
#[component]
fn MobileOverlay(mobile_menu_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div 
            class="mobile-overlay"
            class:active=move || mobile_menu_open.get()
            on:click=move |_| mobile_menu_open.set(false)
        ></div>
    }
}

// Sidebar Component
#[component]
fn Sidebar(collapsed: RwSignal<bool>, mobile_menu_open: RwSignal<bool>) -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();
    
    let is_active = move |path: &str| {
        pathname() == path
    };

    view! {
        <aside 
            class="sidebar"
            class:collapsed=move || collapsed.get()
            class:mobile-active=move || mobile_menu_open.get()
        >
            <div class="sidebar-header">
                <a href="/" class="sidebar-logo">
                    <svg class="heroicon-lg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.5 12h-15m0 0l6.75 6.75M4.5 12l6.75-6.75"/>
                    </svg>
                    <span class="logo-text">"Loco Platform"</span>
                </a>
                <button 
                    class="sidebar-toggle"
                    on:click=move |_| collapsed.update(|v| *v = !*v)
                >
                    <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7"/>
                    </svg>
                </button>
            </div>

            <nav class="sidebar-nav">
                <div class="nav-section">
                    <div class="nav-section-title">"Main"</div>
                    <a href="/" class={move || if is_active("/") { "nav-item active" } else { "nav-item" }}>
                        <div class="nav-item-icon">
                            <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"/>
                            </svg>
                        </div>
                        <span class="nav-item-text">"Home"</span>
                        <div class="nav-tooltip">"Home"</div>
                    </a>
                    
                    <a href="/dashboard" class={move || if is_active("/dashboard") { "nav-item active" } else { "nav-item" }}>
                        <div class="nav-item-icon">
                            <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z"/>
                            </svg>
                        </div>
                        <span class="nav-item-text">"Dashboard"</span>
                        <div class="nav-tooltip">"Dashboard"</div>
                    </a>
                </div>

                <div class="nav-section">
                    <div class="nav-section-title">"Jobs"</div>
                    <a href="/jobs" class={move || if is_active("/jobs") { "nav-item active" } else { "nav-item" }}>
                        <div class="nav-item-icon">
                            <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.25 14.15v4.25c0 1.094-.787 2.036-1.872 2.18-2.087.277-4.216.42-6.378.42s-4.291-.143-6.378-.42c-1.085-.144-1.872-1.086-1.872-2.18v-4.25m16.5 0a2.18 2.18 0 00.75-1.661V8.706c0-1.081-.768-2.015-1.837-2.175a48.114 48.114 0 00-3.413-.387m4.5 8.006c-.194.165-.42.295-.673.38A23.978 23.978 0 0112 15.75c-2.648 0-5.195-.429-7.577-1.22a2.016 2.016 0 01-.673-.38m0 0A2.18 2.18 0 013 12.489V8.706c0-1.081.768-2.015 1.837-2.175a48.111 48.111 0 013.413-.387m7.5 0V5.25A2.25 2.25 0 0013.5 3h-3a2.25 2.25 0 00-2.25 2.25v.894m7.5 0a48.667 48.667 0 00-7.5 0M12 12.75h.008v.008H12v-.008z"/>
                            </svg>
                        </div>
                        <span class="nav-item-text">"Browse Jobs"</span>
                        <div class="nav-tooltip">"Browse Jobs"</div>
                    </a>
                    
                    <a href="/map" class={move || if is_active("/map") { "nav-item active" } else { "nav-item" }}>
                        <div class="nav-item-icon">
                            <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10.5a3 3 0 11-6 0 3 3 0 016 0z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.5 21C19.5 17.134 16.142 14 12 14s-7.5 3.134-7.5 7"/>
                            </svg>
                        </div>
                        <span class="nav-item-text">"Map View"</span>
                        <div class="nav-tooltip">"Map View"</div>
                    </a>
                    
                    <a href="/create-job" class={move || if is_active("/create-job") { "nav-item active" } else { "nav-item" }}>
                        <div class="nav-item-icon">
                            <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.5v15m7.5-7.5h-15"/>
                            </svg>
                        </div>
                        <span class="nav-item-text">"Post Job"</span>
                        <div class="nav-tooltip">"Post Job"</div>
                    </a>
                </div>

                <div class="nav-section">
                    <div class="nav-section-title">"Account"</div>
                    <a href="/profile" class={move || if is_active("/profile") { "nav-item active" } else { "nav-item" }}>
                        <div class="nav-item-icon">
                            <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.982 18.725A7.488 7.488 0 0012 15.75a7.488 7.488 0 00-5.982 2.975m11.963 0a9 9 0 10-11.963 0m11.963 0A8.966 8.966 0 0112 21a8.966 8.966 0 01-5.982-2.275M15 9.75a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                        <span class="nav-item-text">"Profile"</span>
                        <div class="nav-tooltip">"Profile"</div>
                    </a>
                    
                    <a href="/admin" class={move || if is_active("/admin") { "nav-item active" } else { "nav-item" }}>
                        <div class="nav-item-icon">
                            <svg class="heroicon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.559.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.894.149c-.424.07-.764.383-.929.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.398.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.272-.806.108-1.204-.165-.397-.506-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.108-1.204l-.526-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                        <span class="nav-item-text">"Admin"</span>
                        <div class="nav-tooltip">"Admin"</div>
                    </a>
                </div>
            </nav>
        </aside>
    }
}

// Dashboard Page Component
#[component]
fn DashboardPage() -> impl IntoView {
    // State for API data
    let total_jobs = RwSignal::new(156);
    let active_applications = RwSignal::new(42);
    let urgent_jobs = RwSignal::new(8);
    let new_messages = RwSignal::new(5);

    view! {
        <div class="container mx-auto px-6 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Dashboard"</h1>
            
            // Stats Grid
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <StatsWidget 
                    title="Total Jobs"
                    value=Signal::derive(move || total_jobs.get().to_string())
                    icon="briefcase"
                    color="blue"
                />
                <StatsWidget 
                    title="Active Applications"
                    value=Signal::derive(move || active_applications.get().to_string())
                    icon="document"
                    color="green"
                />
                <StatsWidget 
                    title="Urgent Jobs"
                    value=Signal::derive(move || urgent_jobs.get().to_string())
                    icon="exclamation"
                    color="red"
                />
                <StatsWidget 
                    title="New Messages"
                    value=Signal::derive(move || new_messages.get().to_string())
                    icon="envelope"
                    color="purple"
                />
            </div>

            // Recent Jobs Section
            <div class="glass rounded-xl p-6 mb-8">
                <h2 class="text-xl font-semibold text-gray-900 mb-4">"Recent Jobs"</h2>
                <div class="space-y-4">
                    <JobListItem 
                        title="Senior Pharmacist - North Adelaide"
                        pharmacy="Women's and Children's Hospital"
                        location="North Adelaide, SA"
                        posted="2 hours ago"
                    />
                    <JobListItem 
                        title="Pharmacy Assistant - Marion"
                        pharmacy="Westfield Medical Pharmacy"
                        location="Marion, SA"
                        posted="5 hours ago"
                    />
                </div>
            </div>
        </div>
    }
}

// Stats Widget Component
#[component]
fn StatsWidget(
    title: &'static str,
    #[prop(into)] value: Signal<String>,
    _icon: &'static str,
    color: &'static str,
) -> impl IntoView {
    let bg_color = match color {
        "blue" => "bg-blue-50",
        "green" => "bg-green-50",
        "red" => "bg-red-50",
        "purple" => "bg-purple-50",
        _ => "bg-gray-50",
    };
    
    let text_color = match color {
        "blue" => "text-blue-600",
        "green" => "text-green-600",
        "red" => "text-red-600",
        "purple" => "text-purple-600",
        _ => "text-gray-600",
    };

    view! {
        <div class="glass rounded-xl p-6">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm text-gray-600">{title}</p>
                    <p class="text-3xl font-bold mt-1">{value}</p>
                </div>
                <div class={format!("{} {} rounded-full p-3", bg_color, text_color)}>
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.5v15m7.5-7.5h-15"/>
                    </svg>
                </div>
            </div>
        </div>
    }
}

// Job List Item Component
#[component]
fn JobListItem(
    title: &'static str,
    pharmacy: &'static str,
    location: &'static str,
    posted: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between py-3 border-b border-gray-200 last:border-0">
            <div>
                <h3 class="font-semibold text-gray-900">{title}</h3>
                <p class="text-sm text-gray-600">{pharmacy} " • " {location}</p>
            </div>
            <div class="text-right">
                <p class="text-sm text-gray-500">{posted}</p>
                <a href="/jobs" class="text-sm text-blue-600 hover:text-blue-800">"View Details"</a>
            </div>
        </div>
    }
}

// Map Page Component
#[component]
fn MapPage() -> impl IntoView {
    view! {
        <div class="h-screen relative">
            <div class="absolute top-4 left-4 z-10 glass rounded-xl p-4 max-w-md">
                <h2 class="text-lg font-semibold mb-3">"Job Locations"</h2>
                <div class="space-y-2">
                    <div class="flex items-center space-x-2">
                        <div class="w-3 h-3 bg-blue-600 rounded-full"></div>
                        <span class="text-sm">"Regular Jobs"</span>
                    </div>
                    <div class="flex items-center space-x-2">
                        <div class="w-3 h-3 bg-red-600 rounded-full"></div>
                        <span class="text-sm">"Urgent Jobs"</span>
                    </div>
                </div>
            </div>
            
            <div id="map" class="w-full h-full bg-gray-200 flex items-center justify-center">
                <div class="text-center">
                    <svg class="w-16 h-16 mx-auto text-gray-400 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7"/>
                    </svg>
                    <p class="text-gray-600">"Interactive map will be loaded here"</p>
                    <p class="text-sm text-gray-500 mt-2">"Requires Mapbox integration"</p>
                </div>
            </div>
        </div>
    }
}

// Profile Page Component
#[component]
fn ProfilePage() -> impl IntoView {
    let active_tab = RwSignal::new("personal");

    view! {
        <div class="container mx-auto px-6 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"My Profile"</h1>
            
            // Profile Header
            <div class="glass rounded-xl p-6 mb-8">
                <div class="flex items-center space-x-6">
                    <div class="w-24 h-24 bg-gray-300 rounded-full flex items-center justify-center">
                        <svg class="w-12 h-12 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="text-2xl font-semibold">"John Smith"</h2>
                        <p class="text-gray-600">"john.smith@example.com"</p>
                        <p class="text-sm text-gray-500 mt-1">"Member since January 2024"</p>
                    </div>
                </div>
            </div>

            // Tabs
            <div class="glass rounded-xl overflow-hidden">
                <div class="border-b border-gray-200">
                    <nav class="flex">
                        <button 
                            class="px-6 py-3 text-sm font-medium border-b-2 transition-colors"
                            class:border-blue-500=move || active_tab.get() == "personal"
                            class:text-blue-600=move || active_tab.get() == "personal"
                            class:border-transparent=move || active_tab.get() != "personal"
                            class:text-gray-500=move || active_tab.get() != "personal"
                            on:click=move |_| active_tab.set("personal")
                        >
                            "Personal Information"
                        </button>
                    </nav>
                </div>
                
                <div class="p-6">
                    <form class="space-y-4">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-1">"First Name"</label>
                                <input type="text" value="John" class="input" />
                            </div>
                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-1">"Last Name"</label>
                                <input type="text" value="Smith" class="input" />
                            </div>
                        </div>
                        <button type="submit" class="btn btn-primary">"Save Changes"</button>
                    </form>
                </div>
            </div>
        </div>
    }
}

// Admin Page Component
#[component]
fn AdminPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-6 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Admin Dashboard"</h1>
            
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="glass rounded-xl p-6">
                    <h2 class="text-xl font-semibold mb-4">"User Management"</h2>
                    <div class="space-y-3">
                        <div class="flex justify-between items-center">
                            <span>"Total Users"</span>
                            <span class="font-semibold">"1,234"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span>"Active Today"</span>
                            <span class="font-semibold">"89"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span>"New This Week"</span>
                            <span class="font-semibold">"45"</span>
                        </div>
                    </div>
                    <button class="btn btn-primary w-full mt-4">"Manage Users"</button>
                </div>
                
                <div class="glass rounded-xl p-6">
                    <h2 class="text-xl font-semibold mb-4">"Job Management"</h2>
                    <div class="space-y-3">
                        <div class="flex justify-between items-center">
                            <span>"Active Jobs"</span>
                            <span class="font-semibold">"156"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span>"Pending Approval"</span>
                            <span class="font-semibold text-orange-600">"12"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span>"Expired"</span>
                            <span class="font-semibold">"34"</span>
                        </div>
                    </div>
                    <button class="btn btn-primary w-full mt-4">"Review Jobs"</button>
                </div>
                
                <div class="glass rounded-xl p-6">
                    <h2 class="text-xl font-semibold mb-4">"System Health"</h2>
                    <div class="space-y-3">
                        <div class="flex justify-between items-center">
                            <span>"Server Status"</span>
                            <span class="text-green-600 font-semibold">"Healthy"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span>"Database"</span>
                            <span class="text-green-600 font-semibold">"Connected"</span>
                        </div>
                        <div class="flex justify-between items-center">
                            <span>"API Uptime"</span>
                            <span class="font-semibold">"99.9%"</span>
                        </div>
                    </div>
                    <button class="btn btn-secondary w-full mt-4">"View Logs"</button>
                </div>
            </div>
        </div>
    }
}

// Create Job Page Component
#[component]
fn CreateJobPage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-6 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-8">"Post a New Job"</h1>
            
            <div class="glass rounded-xl p-8 max-w-4xl mx-auto">
                <form class="space-y-6">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">"Job Title"</label>
                        <input type="text" placeholder="e.g., Senior Pharmacist" class="input" />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">"Pharmacy Name"</label>
                        <input type="text" placeholder="Enter pharmacy name" class="input" />
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">"Location"</label>
                            <input type="text" placeholder="e.g., Sydney, NSW" class="input" />
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-2">"Hourly Rate"</label>
                            <input type="text" placeholder="e.g., $65.00" class="input" />
                        </div>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">"Job Description"</label>
                        <textarea rows="5" placeholder="Describe the role and responsibilities..." class="input"></textarea>
                    </div>
                    
                    <div class="flex items-center space-x-2">
                        <input type="checkbox" id="urgent" />
                        <label for="urgent" class="text-sm">"Mark as urgent"</label>
                    </div>
                    
                    <div class="flex space-x-4">
                        <button type="submit" class="btn btn-primary">"Post Job"</button>
                        <button type="button" class="btn btn-secondary">"Save as Draft"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}

// Login Page Component
#[component]
fn LoginPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center px-6">
            <div class="glass rounded-xl p-8 w-full max-w-md">
                <h1 class="text-3xl font-bold text-center mb-8">"Welcome Back"</h1>
                
                <form class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Email"</label>
                        <input type="email" placeholder="Enter your email" class="input" />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Password"</label>
                        <input type="password" placeholder="Enter your password" class="input" />
                    </div>
                    
                    <div class="flex items-center justify-between">
                        <label class="flex items-center">
                            <input type="checkbox" class="mr-2" />
                            <span class="text-sm">"Remember me"</span>
                        </label>
                        <a href="#" class="text-sm text-blue-600 hover:text-blue-800">"Forgot password?"</a>
                    </div>
                    
                    <button type="submit" class="btn btn-primary w-full">"Sign In"</button>
                </form>
                
                <p class="text-center mt-6 text-sm text-gray-600">
                    "Don't have an account? "
                    <a href="/register" class="text-blue-600 hover:text-blue-800">"Sign up"</a>
                </p>
            </div>
        </div>
    }
}

// Register Page Component
#[component]
fn RegisterPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center px-6">
            <div class="glass rounded-xl p-8 w-full max-w-md">
                <h1 class="text-3xl font-bold text-center mb-8">"Create Account"</h1>
                
                <form class="space-y-4">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">"First Name"</label>
                            <input type="text" placeholder="John" class="input" />
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">"Last Name"</label>
                            <input type="text" placeholder="Smith" class="input" />
                        </div>
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Email"</label>
                        <input type="email" placeholder="Enter your email" class="input" />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Password"</label>
                        <input type="password" placeholder="Create a password" class="input" />
                    </div>
                    
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Confirm Password"</label>
                        <input type="password" placeholder="Confirm your password" class="input" />
                    </div>
                    
                    <div class="flex items-start">
                        <input type="checkbox" id="terms" class="mt-1 mr-2" />
                        <label for="terms" class="text-sm text-gray-600">
                            "I agree to the "<a href="#" class="text-blue-600 hover:text-blue-800">"Terms and Conditions"</a>
                        </label>
                    </div>
                    
                    <button type="submit" class="btn btn-primary w-full">"Create Account"</button>
                </form>
                
                <p class="text-center mt-6 text-sm text-gray-600">
                    "Already have an account? "
                    <a href="/login" class="text-blue-600 hover:text-blue-800">"Sign in"</a>
                </p>
            </div>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-gray-900 mb-4">"404"</h1>
                <p class="text-xl text-gray-600 mb-8">"Page not found"</p>
                <a href="/" class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg">
                    "Go Home"
                </a>
            </div>
        </div>
    }
}