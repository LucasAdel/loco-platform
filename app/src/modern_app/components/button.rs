use leptos::*;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Danger,
    Success,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Button(
    #[prop(into, optional)] variant: MaybeSignal<ButtonVariant>,
    #[prop(into, optional)] size: MaybeSignal<ButtonSize>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] full_width: bool,
    #[prop(optional)] on_click: Option<Box<dyn Fn(ev::MouseEvent) + 'static>>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(MaybeSignal::Static(ButtonVariant::Primary));
    let size = size.unwrap_or(MaybeSignal::Static(ButtonSize::Medium));
    
    let base_classes = "inline-flex items-center justify-center font-medium rounded-xl transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100";
    
    let variant_classes = move || match variant.get() {
        ButtonVariant::Primary => "bg-gradient-to-r from-blue-600 to-blue-700 text-white hover:from-blue-700 hover:to-blue-800 focus:ring-blue-500 shadow-lg shadow-blue-500/25",
        ButtonVariant::Secondary => "bg-white/70 backdrop-blur-xl border border-gray-200 text-gray-700 hover:bg-white/90 hover:border-gray-300 focus:ring-gray-500",
        ButtonVariant::Ghost => "bg-transparent text-gray-600 hover:bg-gray-100/50 hover:text-gray-900 focus:ring-gray-500",
        ButtonVariant::Danger => "bg-gradient-to-r from-red-600 to-red-700 text-white hover:from-red-700 hover:to-red-800 focus:ring-red-500 shadow-lg shadow-red-500/25",
        ButtonVariant::Success => "bg-gradient-to-r from-green-600 to-green-700 text-white hover:from-green-700 hover:to-green-800 focus:ring-green-500 shadow-lg shadow-green-500/25",
    };
    
    let size_classes = move || match size.get() {
        ButtonSize::Small => "px-3 py-1.5 text-sm gap-1.5",
        ButtonSize::Medium => "px-4 py-2 text-base gap-2",
        ButtonSize::Large => "px-6 py-3 text-lg gap-2.5",
    };
    
    let width_class = if full_width { "w-full" } else { "" };
    
    view! {
        <button
            class=move || format!("{} {} {} {}", base_classes, variant_classes(), size_classes(), width_class)
            disabled=disabled
            on:click=move |ev| {
                if let Some(handler) = &on_click {
                    handler(ev);
                }
            }
        >
            {children()}
        </button>
    }
}