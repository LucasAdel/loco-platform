use leptos::*;
use leptos::prelude::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);
    let (user_menu_open, set_user_menu_open) = create_signal(false);

    view! {
        <header class="bg-white/70 backdrop-blur-xl border-b border-white/20 sticky top-0 z-50 shadow-sm">
            <div class="px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    // Logo and brand
                    <div class="flex items-center">
                        <A href="/" class="flex items-center space-x-2">
                            <div class="w-8 h-8 bg-gradient-to-br from-tiffany-500 to-blue-600 rounded-lg flex items-center justify-center">
                                <span class="text-white font-bold text-sm">"L"</span>
                            </div>
                            <div class="flex flex-col">
                                <span class="text-xl font-bold text-gray-900">"Loco"</span>
                                <span class="text-xs text-gray-500 -mt-1">"Platform"</span>
                            </div>
                        </A>
                    </div>

                    // Desktop navigation
                    <nav class="hidden lg:flex items-center space-x-6">
                        <A
                            href="/dashboard"
                            class="text-gray-700 hover:text-tiffany-600 font-medium transition-colors px-3 py-2 rounded-lg hover:bg-tiffany-50"
                        >
                            "Dashboard"
                        </A>
                        <A
                            href="/jobs"
                            class="text-gray-700 hover:text-tiffany-600 font-medium transition-colors px-3 py-2 rounded-lg hover:bg-tiffany-50"
                        >
                            "Jobs"
                        </A>
                        <A
                            href="/map"
                            class="text-gray-700 hover:text-tiffany-600 font-medium transition-colors px-3 py-2 rounded-lg hover:bg-tiffany-50"
                        >
                            "Map"
                        </A>
                        <A
                            href="/applications"
                            class="text-gray-700 hover:text-tiffany-600 font-medium transition-colors px-3 py-2 rounded-lg hover:bg-tiffany-50"
                        >
                            "Applications"
                        </A>
                        <A
                            href="/profile"
                            class="text-gray-700 hover:text-tiffany-600 font-medium transition-colors px-3 py-2 rounded-lg hover:bg-tiffany-50"
                        >
                            "Profile"
                        </A>
                        
                        // Quick Actions
                        <div class="flex items-center space-x-3 ml-4 pl-4 border-l border-gray-200">
                            <A
                                href="/jobs/create"
                                class="inline-flex items-center px-4 py-2 bg-gradient-to-r from-tiffany-600 to-blue-600 text-white font-medium rounded-lg hover:from-tiffany-700 hover:to-blue-700 transition-all duration-200 transform hover:scale-105 shadow-sm"
                            >
                                <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                                </svg>
                                "Post Job"
                            </A>
                            
                            // User menu
                            <div class="relative">
                                <button
                                    class="flex items-center space-x-2 p-2 rounded-lg hover:bg-gray-100 transition-colors"
                                    on:click=move |_| set_user_menu_open.update(|v| *v = !*v)
                                >
                                    <div class="w-8 h-8 bg-gradient-to-br from-purple-500 to-pink-500 rounded-full flex items-center justify-center">
                                        <span class="text-white font-semibold text-sm">"U"</span>
                                    </div>
                                    <svg class="w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                    </svg>
                                </button>
                                
                                // User dropdown
                                <Show when=move || user_menu_open.get()>
                                    <div class="absolute right-0 mt-2 w-56 bg-white rounded-xl shadow-lg border border-gray-100 py-2">
                                        <A href="/profile" class="flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-50">
                                            <svg class="w-4 h-4 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                                            </svg>
                                            "My Profile"
                                        </A>
                                        <A href="/settings" class="flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-50">
                                            <svg class="w-4 h-4 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                            </svg>
                                            "Settings"
                                        </A>
                                        <A href="/admin" class="flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-50">
                                            <svg class="w-4 h-4 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
                                            </svg>
                                            "Admin"
                                        </A>
                                        <div class="border-t border-gray-100 my-2"></div>
                                        <button class="flex items-center w-full px-4 py-2 text-sm text-red-600 hover:bg-red-50">
                                            <svg class="w-4 h-4 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                                            </svg>
                                            "Sign Out"
                                        </button>
                                    </div>
                                </Show>
                            </div>
                        </div>
                    </nav>

                    // Mobile menu button
                    <button
                        class="lg:hidden p-2 rounded-md text-gray-700 hover:bg-gray-100"
                        on:click=move |_| set_menu_open.update(|v| *v = !*v)
                    >
                        <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d=if menu_open.get() {
                                    "M6 18L18 6M6 6l12 12"
                                } else {
                                    "M4 6h16M4 12h16M4 18h16"
                                }
                            />
                        </svg>
                    </button>
                </div>
            </div>

            // Mobile menu
            <Show when=move || menu_open.get()>
                <div class="lg:hidden border-t border-gray-200 bg-white/70 backdrop-blur-xl">
                    <nav class="px-4 pt-2 pb-3 space-y-1">
                        <A
                            href="/dashboard"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-tiffany-600 hover:bg-tiffany-50"
                        >
                            "Dashboard"
                        </A>
                        <A
                            href="/jobs"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-tiffany-600 hover:bg-tiffany-50"
                        >
                            "Jobs"
                        </A>
                        <A
                            href="/map"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-tiffany-600 hover:bg-tiffany-50"
                        >
                            "Map"
                        </A>
                        <A
                            href="/applications"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-tiffany-600 hover:bg-tiffany-50"
                        >
                            "Applications"
                        </A>
                        <A
                            href="/profile"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-tiffany-600 hover:bg-tiffany-50"
                        >
                            "Profile"
                        </A>
                        <A
                            href="/settings"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-tiffany-600 hover:bg-tiffany-50"
                        >
                            "Settings"
                        </A>
                        <A
                            href="/admin"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-tiffany-600 hover:bg-tiffany-50"
                        >
                            "Admin"
                        </A>
                        <div class="border-t border-gray-200 my-2"></div>
                        <A
                            href="/jobs/create"
                            class="block px-3 py-2 rounded-md text-base font-medium bg-gradient-to-r from-tiffany-600 to-blue-600 text-white"
                        >
                            "+ Post Job"
                        </A>
                    </nav>
                </div>
            </Show>
        </header>
    }
}