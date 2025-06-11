use leptos::*;
use leptos::prelude::*;
use leptos_router::*;
use crate::providers::auth_provider::use_auth;

/// Protected route component that requires authentication
#[component]
pub fn ProtectedRoute(
    /// The content to show when authenticated
    children: ChildrenFn,
    
    /// Optional fallback path when not authenticated
    #[prop(default = "/login")]
    redirect_to: &'static str,
    
    /// Optional required role
    #[prop(optional)]
    required_role: Option<&'static str>,
    
    /// Optional loading component
    #[prop(optional)]
    loading: Option<View>,
) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let loading_view = loading.clone();
    
    // Check authentication and redirect if needed
    create_effect(move |_| {
        let auth_state = auth.auth_state.get();
        
        if !auth_state.is_authenticated() {
            // Not authenticated, redirect to login
            navigate(redirect_to, Default::default());
        } else if let Some(role) = required_role {
            // Check if user has required role
            if let Some(user) = auth_state.user() {
                if user.role.as_str() != role {
                    // User doesn't have required role, redirect to home
                    navigate("/", Default::default());
                }
            }
        }
    });
    
    view! {
        <Show
            when=move || auth.auth_state.get().is_authenticated()
            fallback=move || {
                if let Some(view) = &loading_view {
                    view.clone()
                } else {
                    view! {
                        <div class="flex items-center justify-center min-h-screen">
                            <div class="text-center">
                                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
                                <p class="mt-4 text-gray-600">Loading...</p>
                            </div>
                        </div>
                    }.into_view()
                }
            }
        >
            {children()}
        </Show>
    }
}

/// Navigation guard hook for programmatic checks
pub fn use_auth_guard() -> impl Fn() -> bool {
    let auth = use_auth();
    
    move || auth.auth_state.get().is_authenticated()
}

/// Role-based guard hook
pub fn use_role_guard(required_role: &'static str) -> impl Fn() -> bool {
    let auth = use_auth();
    
    move || {
        let auth_state = auth.auth_state.get();
        if let Some(user) = auth_state.user() {
            user.role.as_str() == required_role
        } else {
            false
        }
    }
}

/// Guest route component that redirects authenticated users
#[component]
pub fn GuestRoute(
    /// The content to show when not authenticated
    children: ChildrenFn,
    
    /// Path to redirect authenticated users to
    #[prop(default = "/dashboard")]
    redirect_to: &'static str,
) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    
    // Redirect if authenticated
    create_effect(move |_| {
        if auth.auth_state.get().is_authenticated() {
            navigate(redirect_to, Default::default());
        }
    });
    
    view! {
        <Show
            when=move || !auth.auth_state.get().is_authenticated()
            fallback=move || view! {
                <div class="flex items-center justify-center min-h-screen">
                    <p class="text-gray-600">Redirecting...</p>
                </div>
            }
        >
            {children()}
        </Show>
    }
}

/// Route guard wrapper for specific permissions
#[component]
pub fn PermissionGuard(
    /// Required permission
    permission: &'static str,
    
    /// Content to show when permission is granted
    children: ChildrenFn,
    
    /// Optional fallback content
    #[prop(optional)]
    fallback: Option<View>,
) -> impl IntoView {
    let auth = use_auth();
    
    let has_permission = move || {
        let auth_state = auth.auth_state.get();
        if let Some(user) = auth_state.user() {
            // Check if user has the required permission
            // This is a simplified check - in production, you'd check against a permissions list
            match permission {
                "admin" => user.role.as_str() == "admin" || user.role.as_str() == "super_admin",
                "edit_jobs" => ["admin", "super_admin", "employer"].contains(&user.role.as_str()),
                "view_applications" => ["admin", "super_admin", "employer", "professional"].contains(&user.role.as_str()),
                _ => false,
            }
        } else {
            false
        }
    };
    
    view! {
        <Show
            when=has_permission
            fallback=move || {
                if let Some(fb) = &fallback {
                    fb.clone()
                } else {
                    view! {
                        <div class="text-center py-8">
                            <div class="inline-flex items-center justify-center w-16 h-16 bg-red-100 rounded-full mb-4">
                                <svg class="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                        d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                </svg>
                            </div>
                            <h3 class="text-lg font-semibold text-gray-900 mb-2">Access Denied</h3>
                            <p class="text-gray-600">"You don't have permission to access this resource."</p>
                        </div>
                    }.into_view()
                }
            }
        >
            {children()}
        </Show>
    }
}

/// Navigation link with auth check
#[component]
pub fn AuthLink(
    /// Link destination
    href: &'static str,
    
    /// Link text
    children: ChildrenFn,
    
    /// Whether to show only when authenticated
    #[prop(default = true)]
    auth_required: bool,
    
    /// Optional required role
    #[prop(optional)]
    required_role: Option<&'static str>,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
    
    /// Active class
    #[prop(optional)]
    active_class: Option<&'static str>,
) -> impl IntoView {
    let auth = use_auth();
    
    let should_show = move || {
        let auth_state = auth.auth_state.get();
        
        if auth_required && !auth_state.is_authenticated() {
            return false;
        }
        
        if let Some(role) = required_role {
            if let Some(user) = auth_state.user() {
                return user.role.as_str() == role;
            }
            return false;
        }
        
        true
    };
    
    view! {
        <Show when=should_show>
            <A 
                href=href 
                class=class.unwrap_or("")
                active_class=active_class.unwrap_or("")
            >
                {children()}
            </A>
        </Show>
    }
}

/// Redirect component for conditional navigation
#[component]
pub fn ConditionalRedirect(
    /// Condition to check
    when: Box<dyn Fn() -> bool>,
    
    /// Path to redirect to
    to: &'static str,
    
    /// Optional fallback content
    #[prop(optional)]
    fallback: Option<ChildrenFn>,
) -> impl IntoView {
    let navigate = use_navigate();
    
    create_effect(move |_| {
        if when() {
            navigate(to, Default::default());
        }
    });
    
    let fallback_content = fallback.map(|fb| fb()).unwrap_or_else(|| view! {
        <div class="flex items-center justify-center p-4">
            <p class="text-gray-600">No content available</p>
        </div>
    }.into_view());
    
    view! {
        <Show
            when=move || !when()
            fallback=move || view! {
                <div class="flex items-center justify-center p-4">
                    <p class="text-gray-600">Redirecting...</p>
                </div>
            }
        >
            {fallback_content}
        </Show>
    }
}