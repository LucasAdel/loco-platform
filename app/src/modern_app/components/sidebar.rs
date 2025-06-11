use leptos::*;
use leptos::prelude::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();
    let collapsed = RwSignal::new(false);
    
    let nav_items = vec![
        NavItem { path: "/dashboard", label: "Dashboard", icon: "🏠" },
        NavItem { path: "/dashboard/jobs", label: "Jobs", icon: "💼" },
        NavItem { path: "/dashboard/map", label: "Map", icon: "🗺️" },
        NavItem { path: "/dashboard/applications", label: "Applications", icon: "📋" },
        NavItem { path: "/dashboard/messages", label: "Messages", icon: "💬" },
        NavItem { path: "/dashboard/profile", label: "Profile", icon: "👤" },
    ];
    
    view! {
        <aside class=move || format!(
            "fixed inset-y-0 left-0 z-50 bg-white/80 backdrop-blur-xl border-r border-gray-200/50 transition-all duration-300 {}",
            if collapsed.get() { "w-16" } else { "w-64" }
        )>
            <div class="flex h-full flex-col">
                // Logo section
                <div class="flex h-16 items-center justify-between px-4 border-b border-gray-200/50">
                    <a href="/dashboard" class="flex items-center gap-2">
                        <div class="w-10 h-10 bg-gradient-to-br from-blue-600 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-lg">
                            "L"
                        </div>
                        {move || (!collapsed.get()).then(|| view! {
                            <span class="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                                "Loco Platform"
                            </span>
                        })}
                    </a>
                    <button
                        on:click=move |_| collapsed.update(|c| *c = !*c)
                        class="p-1.5 rounded-lg hover:bg-gray-100/50 transition-colors"
                    >
                        <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                        </svg>
                    </button>
                </div>
                
                // Navigation
                <nav class="flex-1 space-y-1 p-4">
                    {nav_items.into_iter().map(|item| {
                        let is_active = move || pathname().starts_with(item.path);
                        view! {
                            <a
                                href=item.path
                                class=move || format!(
                                    "flex items-center gap-3 px-3 py-2.5 rounded-xl transition-all duration-200 {}",
                                    if is_active() {
                                        "bg-gradient-to-r from-blue-600 to-purple-600 text-white shadow-lg shadow-blue-500/25"
                                    } else {
                                        "text-gray-600 hover:bg-gray-100/50 hover:text-gray-900"
                                    }
                                )
                                title=item.label
                            >
                                <span class="text-xl">{item.icon}</span>
                                {move || (!collapsed.get()).then(|| view! {
                                    <span class="font-medium">{item.label}</span>
                                })}
                            </a>
                        }
                    }).collect::<Vec<_>>()}
                </nav>
                
                // User section
                <div class="border-t border-gray-200/50 p-4">
                    <div class=move || format!(
                        "flex items-center gap-3 {}",
                        if collapsed.get() { "justify-center" } else { "" }
                    )>
                        <div class="w-10 h-10 bg-gradient-to-br from-green-400 to-blue-500 rounded-full flex items-center justify-center text-white font-bold">
                            "U"
                        </div>
                        {move || (!collapsed.get()).then(|| view! {
                            <div class="flex-1">
                                <p class="text-sm font-medium text-gray-900">"User Name"</p>
                                <p class="text-xs text-gray-500">"user@example.com"</p>
                            </div>
                        })}
                    </div>
                </div>
            </div>
        </aside>
    }
}

struct NavItem {
    path: &'static str,
    label: &'static str,
    icon: &'static str,
}