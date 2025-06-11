use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use leptos_router::components::*;

mod components;
mod pages;
mod contexts;

use contexts::{AuthProvider, ThemeProvider};
use pages::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <ThemeProvider>
                <AuthProvider>
                    <Routes fallback=|| view! { <NotFound/> }>
                        // Public routes
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/login") view=LoginPage/>
                        <Route path=path!("/register") view=RegisterPage/>
                        
                        // Protected routes with layout
                        <ProtectedRoute path=path!("/dashboard") view=DashboardLayout>
                            <Route path=path!("") view=DashboardPage/>
                            <Route path=path!("/jobs") view=JobsPage/>
                            <Route path=path!("/jobs/:id") view=JobDetailPage/>
                            <Route path=path!("/map") view=MapPage/>
                            <Route path=path!("/profile") view=ProfilePage/>
                            <Route path=path!("/messages") view=MessagesPage/>
                            <Route path=path!("/applications") view=ApplicationsPage/>
                            <Route path=path!("/create-job") view=CreateJobPage/>
                            <Route path=path!("/admin") view=AdminPage/>
                        </ProtectedRoute>
                    </Routes>
                </AuthProvider>
            </ThemeProvider>
        </Router>
    }
}

#[component]
fn ProtectedRoute<F>(
    path: &'static str,
    view: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> impl IntoView + 'static,
{
    let auth = use_context::<AuthContext>().expect("AuthContext not provided");
    
    move || {
        if auth.is_authenticated.get() {
            view!(<Route path=path view=view>{children()}</Route>)
        } else {
            view!(<Navigate to="/login"/>)
        }
    }
}

#[component]
fn DashboardLayout() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <components::Sidebar/>
            <div class="lg:pl-64">
                <components::Header/>
                <main class="py-6">
                    <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                        <Outlet/>
                    </div>
                </main>
            </div>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-gray-900">"404"</h1>
                <p class="mt-4 text-xl text-gray-600">"Page not found"</p>
                <a href="/" class="mt-6 inline-block px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700">
                    "Go Home"
                </a>
            </div>
        </div>
    }
}