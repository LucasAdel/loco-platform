use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/dashboard" view=DashboardPage/>
                <Route path="/jobs" view=JobsPage/>
                <Route path="/map" view=MapPage/>
                <Route path="/login" view=LoginPage/>
                <Route path="/register" view=RegisterPage/>
                <Route path="/*any" view=NotFoundPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let navigate = use_navigate();
    
    view! {
        <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
            // Header
            <header class="bg-white shadow-sm border-b">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        <h1 class="text-2xl font-bold text-gray-900">"Loco Platform"</h1>
                        <nav class="flex space-x-4">
                            <A href="/login" class="text-gray-600 hover:text-gray-900">"Login"</A>
                            <A href="/register" class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                                "Get Started"
                            </A>
                        </nav>
                    </div>
                </div>
            </header>
            
            // Hero Section
            <section class="py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <h2 class="text-5xl font-bold text-gray-900 mb-6">
                        "Find Your Dream Pharmacy Job"
                    </h2>
                    <p class="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
                        "Connect with top pharmacy employers across Australia. Your next career opportunity is just a click away."
                    </p>
                    <div class="flex justify-center space-x-4">
                        <button
                            on:click=move |_| navigate("/jobs", Default::default())
                            class="px-8 py-3 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700"
                        >
                            "Browse Jobs"
                        </button>
                        <button
                            on:click=move |_| navigate("/register", Default::default())
                            class="px-8 py-3 bg-white text-blue-600 rounded-lg font-semibold border-2 border-blue-600 hover:bg-blue-50"
                        >
                            "Post a Job"
                        </button>
                    </div>
                </div>
            </section>
            
            // Features
            <section class="py-16 bg-white">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <FeatureCard
                            icon="🔍"
                            title="Smart Job Matching"
                            description="Our AI-powered system matches you with jobs that fit your skills and preferences."
                        />
                        <FeatureCard
                            icon="📍"
                            title="Location-Based Search"
                            description="Find opportunities near you with our interactive map and location filters."
                        />
                        <FeatureCard
                            icon="📊"
                            title="Career Insights"
                            description="Get real-time data on salaries, demand, and career growth in your area."
                        />
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
                
                // Stats Grid
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <StatCard title="Active Jobs" value="24" change="+12%" color="blue"/>
                    <StatCard title="Applications" value="156" change="+5%" color="green"/>
                    <StatCard title="Profile Views" value="892" change="+23%" color="purple"/>
                    <StatCard title="Messages" value="12" change="-2%" color="yellow"/>
                </div>
                
                // Recent Activity
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-semibold mb-4">"Recent Activity"</h2>
                    <div class="space-y-4">
                        <ActivityItem
                            title="New application received"
                            description="For Senior Pharmacist position at Sydney Hospital"
                            time="2 hours ago"
                        />
                        <ActivityItem
                            title="Profile viewed"
                            description="By Melbourne Medical Centre"
                            time="5 hours ago"
                        />
                        <ActivityItem
                            title="Job posting expiring soon"
                            description="Clinical Pharmacist position expires in 3 days"
                            time="1 day ago"
                        />
                    </div>
                </div>
            </div>
        </AppLayout>
    }
}

#[component]
fn JobsPage() -> impl IntoView {
    let jobs = vec![
        JobData {
            id: "1",
            title: "Senior Pharmacist",
            company: "Sydney Hospital",
            location: "Sydney, NSW",
            salary: "$95,000 - $115,000",
            job_type: "Full-time",
            posted: "2 days ago",
        },
        JobData {
            id: "2",
            title: "Clinical Pharmacist",
            company: "Melbourne Medical",
            location: "Melbourne, VIC",
            salary: "$85,000 - $105,000",
            job_type: "Full-time",
            posted: "3 days ago",
        },
        JobData {
            id: "3",
            title: "Community Pharmacist",
            company: "Brisbane Pharmacy",
            location: "Brisbane, QLD",
            salary: "$80,000 - $95,000",
            job_type: "Part-time",
            posted: "1 week ago",
        },
    ];
    
    view! {
        <AppLayout>
            <div class="space-y-6">
                <div class="flex justify-between items-center">
                    <h1 class="text-3xl font-bold text-gray-900">"Job Listings"</h1>
                    <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                        "Post a Job"
                    </button>
                </div>
                
                // Search and Filters
                <div class="bg-white rounded-lg shadow p-4">
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                        <input
                            type="text"
                            placeholder="Search jobs..."
                            class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                        <select class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
                            <option>"All Locations"</option>
                            <option>"Sydney"</option>
                            <option>"Melbourne"</option>
                            <option>"Brisbane"</option>
                        </select>
                        <select class="px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
                            <option>"All Types"</option>
                            <option>"Full-time"</option>
                            <option>"Part-time"</option>
                            <option>"Contract"</option>
                        </select>
                        <button class="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200">
                            "More Filters"
                        </button>
                    </div>
                </div>
                
                // Job Listings
                <div class="space-y-4">
                    {jobs.into_iter()
                        .map(|job| view! { <JobCard job=job/> })
                        .collect::<Vec<_>>()}
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
                    <div class="text-center">
                        <div class="text-6xl mb-4">"🗺️"</div>
                        <p class="text-xl text-gray-600">"Interactive map coming soon!"</p>
                        <p class="text-gray-500 mt-2">"Explore pharmacy jobs by location"</p>
                    </div>
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
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-gray-900">"Welcome Back"</h2>
                    <p class="text-gray-600 mt-2">"Sign in to your account"</p>
                </div>
                
                <form class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Email"</label>
                        <input
                            type="email"
                            class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="you@example.com"
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Password"</label>
                        <input
                            type="password"
                            class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="••••••••"
                        />
                    </div>
                    <div class="flex items-center justify-between">
                        <label class="flex items-center">
                            <input type="checkbox" class="mr-2"/>
                            <span class="text-sm text-gray-600">"Remember me"</span>
                        </label>
                        <A href="/forgot-password" class="text-sm text-blue-600 hover:underline">
                            "Forgot password?"
                        </A>
                    </div>
                    <button
                        type="submit"
                        class="w-full bg-blue-600 text-white py-2 rounded-lg font-semibold hover:bg-blue-700"
                    >
                        "Sign In"
                    </button>
                </form>
                
                <p class="text-center text-gray-600 mt-6">
                    "Don't have an account? "
                    <A href="/register" class="text-blue-600 hover:underline">"Sign up"</A>
                </p>
            </div>
        </div>
    }
}

#[component]
fn RegisterPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 flex items-center justify-center">
            <div class="bg-white rounded-lg shadow-lg p-8 w-full max-w-md">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-gray-900">"Create Account"</h2>
                    <p class="text-gray-600 mt-2">"Join the Loco Platform community"</p>
                </div>
                
                <form class="space-y-4">
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">"First Name"</label>
                            <input
                                type="text"
                                class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            />
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-700 mb-1">"Last Name"</label>
                            <input
                                type="text"
                                class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            />
                        </div>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Email"</label>
                        <input
                            type="email"
                            class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="you@example.com"
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"Password"</label>
                        <input
                            type="password"
                            class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="••••••••"
                        />
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1">"I am a..."</label>
                        <select class="w-full px-4 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
                            <option>"Job Seeker"</option>
                            <option>"Employer"</option>
                        </select>
                    </div>
                    <button
                        type="submit"
                        class="w-full bg-blue-600 text-white py-2 rounded-lg font-semibold hover:bg-blue-700"
                    >
                        "Create Account"
                    </button>
                </form>
                
                <p class="text-center text-gray-600 mt-6">
                    "Already have an account? "
                    <A href="/login" class="text-blue-600 hover:underline">"Sign in"</A>
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
                <p class="text-gray-500 mt-2">"The page you're looking for doesn't exist."</p>
                <A href="/" class="inline-block mt-6 px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                    "Go Home"
                </A>
            </div>
        </div>
    }
}

// Layout Component
#[component]
fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            // Header
            <header class="bg-white shadow-sm border-b">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        <div class="flex items-center space-x-8">
                            <A href="/dashboard" class="text-xl font-bold text-gray-900">"Loco Platform"</A>
                            <nav class="hidden md:flex space-x-6">
                                <A href="/dashboard" class="text-gray-600 hover:text-gray-900">"Dashboard"</A>
                                <A href="/jobs" class="text-gray-600 hover:text-gray-900">"Jobs"</A>
                                <A href="/map" class="text-gray-600 hover:text-gray-900">"Map"</A>
                            </nav>
                        </div>
                        <div class="flex items-center space-x-4">
                            <button class="p-2 text-gray-600 hover:text-gray-900">
                                <span class="text-xl">"🔔"</span>
                            </button>
                            <div class="w-10 h-10 bg-blue-600 rounded-full flex items-center justify-center text-white font-semibold">
                                "JD"
                            </div>
                        </div>
                    </div>
                </div>
            </header>
            
            // Main content
            <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                {children()}
            </main>
        </div>
    }
}

// Component Props
#[derive(Clone)]
struct JobData {
    id: &'static str,
    title: &'static str,
    company: &'static str,
    location: &'static str,
    salary: &'static str,
    job_type: &'static str,
    posted: &'static str,
}

// Sub-components
#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="text-center">
            <div class="text-5xl mb-4">{icon}</div>
            <h3 class="text-xl font-semibold mb-2">{title}</h3>
            <p class="text-gray-600">{description}</p>
        </div>
    }
}

#[component]
fn StatCard(title: &'static str, value: &'static str, change: &'static str, color: &'static str) -> impl IntoView {
    let bg_color = format!("bg-{}-100", color);
    let text_color = format!("text-{}-800", color);
    
    view! {
        <div class="bg-white rounded-lg shadow p-6">
            <div class="flex items-center justify-between">
                <div>
                    <p class="text-sm text-gray-600">{title}</p>
                    <p class="text-2xl font-bold text-gray-900 mt-1">{value}</p>
                </div>
                <div class=format!("px-3 py-1 {} {} rounded-full text-sm font-medium", bg_color, text_color)>
                    {change}
                </div>
            </div>
        </div>
    }
}

#[component]
fn ActivityItem(title: &'static str, description: &'static str, time: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-start space-x-3">
            <div class="w-2 h-2 bg-blue-600 rounded-full mt-2"></div>
            <div class="flex-1">
                <p class="font-medium text-gray-900">{title}</p>
                <p class="text-sm text-gray-600">{description}</p>
                <p class="text-xs text-gray-500 mt-1">{time}</p>
            </div>
        </div>
    }
}

#[component]
fn JobCard(job: JobData) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow p-6 hover:shadow-lg transition-shadow cursor-pointer">
            <div class="flex justify-between items-start">
                <div>
                    <h3 class="text-xl font-semibold text-gray-900">{job.title}</h3>
                    <p class="text-gray-600 mt-1">{job.company}</p>
                    <div class="flex items-center space-x-4 mt-3 text-sm text-gray-500">
                        <span class="flex items-center">
                            <span class="mr-1">"📍"</span> {job.location}
                        </span>
                        <span class="flex items-center">
                            <span class="mr-1">"💼"</span> {job.job_type}
                        </span>
                        <span class="flex items-center">
                            <span class="mr-1">"⏰"</span> {job.posted}
                        </span>
                    </div>
                </div>
                <div class="text-right">
                    <p class="text-lg font-semibold text-gray-900">{job.salary}</p>
                    <button class="mt-3 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm">
                        "Apply Now"
                    </button>
                </div>
            </div>
        </div>
    }
}