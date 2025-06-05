pub mod auth_provider;
pub mod tenant_provider;

use leptos::*;

use auth_provider::*;
use tenant_provider::*;

#[component]
pub fn AppProviders(children: Children) -> impl IntoView {
    view! {
        <AuthProvider>
            <TenantProvider>
                {children()}
            </TenantProvider>
        </AuthProvider>
    }
}