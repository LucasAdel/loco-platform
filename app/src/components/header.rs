use leptos::*;
use leptos_router::*;
use crate::theme::ThemeToggle;

#[component]
pub fn Header() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);

    view! {
        <header class="bg-primary border-b border-primary sticky top-0 z-50">
            <div class="px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    // Logo and brand
                    <div class="flex items-center">
                        <A href="/" class="flex items-center">
                            <span class="text-2xl font-bold text-blue-600">Loco</span>
                            <span class="text-2xl font-light text-primary ml-1">Platform</span>
                        </A>
                    </div>

                    // Desktop navigation
                    <nav class="hidden md:flex items-center space-x-8">
                        <A
                            href="/jobs"
                            class="text-gray-700 hover:text-blue-600 font-medium transition-colors"
                            active_class="text-blue-600"
                        >
                            "Jobs"
                        </A>
                        <A
                            href="/map"
                            class="text-gray-700 hover:text-blue-600 font-medium transition-colors"
                            active_class="text-blue-600"
                        >
                            "Map"
                        </A>
                        <A
                            href="/profile"
                            class="text-gray-700 hover:text-blue-600 font-medium transition-colors"
                            active_class="text-blue-600"
                        >
                            "Profile"
                        </A>
                        
                        // Theme toggle
                        <ThemeToggle />
                    </nav>

                    // Mobile menu button
                    <button
                        class="md:hidden p-2 rounded-md text-gray-700 hover:bg-gray-100"
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
                <div class="md:hidden border-t border-gray-200">
                    <nav class="px-2 pt-2 pb-3 space-y-1">
                        <A
                            href="/jobs"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-blue-600 hover:bg-gray-50"
                            active_class="text-blue-600 bg-blue-50"
                        >
                            "Jobs"
                        </A>
                        <A
                            href="/map"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-blue-600 hover:bg-gray-50"
                            active_class="text-blue-600 bg-blue-50"
                        >
                            "Map"
                        </A>
                        <A
                            href="/profile"
                            class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-blue-600 hover:bg-gray-50"
                            active_class="text-blue-600 bg-blue-50"
                        >
                            "Profile"
                        </A>
                    </nav>
                </div>
            </Show>
        </header>
    }
}