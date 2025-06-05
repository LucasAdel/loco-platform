use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let pathname = use_location().pathname;

    let nav_items = vec![
        ("🏠", "Home", "/"),
        ("💼", "Jobs", "/jobs"),
        ("🗺", "Map", "/map"),
        ("👤", "Profile", "/profile"),
        ("⚙️", "Settings", "/settings"),
        ("👥", "Team", "/team"),
        ("📊", "Dashboard", "/dashboard"),
    ];

    view! {
        <aside class="hidden md:flex md:flex-shrink-0">
            <div class="flex flex-col w-64">
                <div class="flex flex-col h-0 flex-1 bg-gray-800">
                    <div class="flex-1 flex flex-col pt-5 pb-4 overflow-y-auto">
                        <nav class="mt-5 flex-1 px-2 space-y-1">
                            {nav_items
                                .into_iter()
                                .map(|(icon, label, href)| {
                                    let is_active = move || pathname.get() == href;
                                    
                                    view! {
                                        <A
                                            href=href
                                            class=move || {
                                                let base = "group flex items-center px-2 py-2 text-sm font-medium rounded-md transition-colors";
                                                if is_active() {
                                                    format!("{} bg-gray-900 text-white", base)
                                                } else {
                                                    format!("{} text-gray-300 hover:bg-gray-700 hover:text-white", base)
                                                }
                                            }
                                        >
                                            <span class="mr-3 text-lg">{icon}</span>
                                            {label}
                                        </A>
                                    }
                                })
                                .collect_view()
                            }
                        </nav>
                    </div>
                </div>
            </div>
        </aside>
    }
}