use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "ssr")]
pub use leptos_axum::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    use tracing_subscriber;
    console_error_panic_hook::set_once();
    tracing_subscriber::fmt::init();
    leptos::mount_to_body(App);
}

pub mod api;
pub mod auth;
pub mod components;
pub mod pages;
pub mod providers;
pub mod utils;
pub mod config;
pub mod theme;

use crate::providers::*;
use crate::theme::ThemeProvider;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/loco-app.css"/>
        <Title text="Loco Platform - Australian Pharmacy Jobs"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        <ThemeProvider>
            <Router>
                <AppProviders>
                    <Routes>
                        <Route path="" view=Layout>
                            <Route path="/" view=pages::Home/>
                            <Route path="/jobs" view=pages::Jobs/>
                            <Route path="/jobs/:id" view=pages::JobDetail/>
                            <Route path="/map" view=pages::Map/>
                            <Route path="/profile" view=pages::Profile/>
                            <Route path="/admin" view=pages::Admin/>
                            
                            // Auth routes
                            <Route path="/login" view=auth::Login/>
                            <Route path="/register" view=auth::Register/>
                            <Route path="/forgot-password" view=auth::ForgotPassword/>
                            
                            // Tenant-specific routes
                            <Route path="/dashboard" view=pages::Dashboard/>
                            <Route path="/settings" view=pages::Settings/>
                            <Route path="/team" view=pages::Team/>
                        </Route>
                        
                        // 404 page
                        <Route path="/*any" view=pages::NotFound/>
                    </Routes>
                </AppProviders>
            </Router>
        </ThemeProvider>
    }
}

#[component]
fn Layout() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <components::Header/>
            <div class="flex">
                <components::Sidebar/>
                <main class="flex-1">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}