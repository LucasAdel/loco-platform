use leptos::*;
use leptos::prelude::*;
use crate::contexts::use_auth;

#[component]
pub fn Header() -> impl IntoView {
    let auth = use_auth();
    let show_dropdown = RwSignal::new(false);
    
    view! {
        <header class="sticky top-0 z-40 bg-white/70 backdrop-blur-xl border-b border-gray-200/50">
            <div class="flex h-16 items-center justify-between px-4 sm:px-6 lg:px-8">
                // Search bar
                <div class="flex flex-1 items-center">
                    <div class="w-full max-w-lg">
                        <label for="search" class="sr-only">"Search"</label>
                        <div class="relative">
                            <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
                                <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                                </svg>
                            </div>
                            <input
                                id="search"
                                name="search"
                                class="block w-full rounded-xl border-0 bg-gray-100/50 py-2 pl-10 pr-3 text-gray-900 placeholder:text-gray-400 focus:bg-white focus:ring-2 focus:ring-blue-500 focus:outline-none sm:text-sm"
                                placeholder="Search jobs, companies..."
                                type="search"
                            />
                        </div>
                    </div>
                </div>
                
                // Right side actions
                <div class="ml-4 flex items-center gap-3">
                    // Notifications
                    <button class="relative p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100/50 rounded-lg transition-colors">
                        <span class="sr-only">"Notifications"</span>
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                        </svg>
                        <span class="absolute top-1 right-1 block h-2 w-2 rounded-full bg-red-500 ring-2 ring-white"></span>
                    </button>
                    
                    // User menu
                    <div class="relative">
                        <button
                            on:click=move |_| show_dropdown.update(|s| *s = !*s)
                            class="flex items-center gap-2 p-2 hover:bg-gray-100/50 rounded-lg transition-colors"
                        >
                            <div class="w-8 h-8 bg-gradient-to-br from-green-400 to-blue-500 rounded-full flex items-center justify-center text-white font-bold text-sm">
                                "U"
                            </div>
                            <svg class="w-4 h-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                            </svg>
                        </button>
                        
                        // Dropdown menu
                        {move || show_dropdown.get().then(|| view! {
                            <div class="absolute right-0 mt-2 w-48 origin-top-right rounded-xl bg-white/90 backdrop-blur-xl shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                                <div class="py-1">
                                    <a href="/dashboard/profile" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100/50">
                                        "Your Profile"
                                    </a>
                                    <a href="/dashboard/settings" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100/50">
                                        "Settings"
                                    </a>
                                    <hr class="my-1 border-gray-200/50"/>
                                    <button
                                        on:click=move |_| {
                                            let auth = auth.clone();
                                            spawn_local(async move {
                                                auth.logout().await;
                                            });
                                        }
                                        class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100/50"
                                    >
                                        "Sign out"
                                    </button>
                                </div>
                            </div>
                        })}
                    </div>
                </div>
            </div>
        </header>
    }
}