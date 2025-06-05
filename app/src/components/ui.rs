use leptos::*;

#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let base_classes = "inline-flex items-center justify-center font-medium rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2";
    
    let variant_classes = match variant {
        ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500",
        ButtonVariant::Secondary => "bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500",
        ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700 focus:ring-red-500",
        ButtonVariant::Ghost => "bg-transparent hover:bg-gray-100 text-gray-700 focus:ring-gray-500",
    };
    
    let size_classes = match size {
        ButtonSize::Small => "px-3 py-1.5 text-sm",
        ButtonSize::Medium => "px-4 py-2 text-base",
        ButtonSize::Large => "px-6 py-3 text-lg",
    };
    
    let disabled_classes = if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };
    
    let classes = format!("{} {} {} {}", base_classes, variant_classes, size_classes, disabled_classes);
    
    view! {
        <button
            class=classes
            disabled=disabled
            on:click=move |ev| {
                if let Some(callback) = &on_click {
                    callback.call(ev);
                }
            }
        >
            {children()}
        </button>
    }
}

#[derive(Default, Clone, Copy)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[derive(Default, Clone, Copy)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let classes = format!("bg-white rounded-lg shadow-md p-6 {}", class);
    
    view! {
        <div class=classes>
            {children()}
        </div>
    }
}

#[component]
pub fn LoadingSpinner(
    #[prop(optional)] size: SpinnerSize,
) -> impl IntoView {
    let size_classes = match size {
        SpinnerSize::Small => "h-4 w-4",
        SpinnerSize::Medium => "h-8 w-8",
        SpinnerSize::Large => "h-12 w-12",
    };
    
    view! {
        <div class="flex justify-center items-center">
            <div class=format!("animate-spin rounded-full border-b-2 border-blue-600 {}", size_classes)></div>
        </div>
    }
}

#[derive(Default, Clone, Copy)]
pub enum SpinnerSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[component]
pub fn Badge(
    children: Children,
    #[prop(optional)] variant: BadgeVariant,
) -> impl IntoView {
    let classes = match variant {
        BadgeVariant::Primary => "bg-blue-100 text-blue-800",
        BadgeVariant::Success => "bg-green-100 text-green-800",
        BadgeVariant::Warning => "bg-yellow-100 text-yellow-800",
        BadgeVariant::Danger => "bg-red-100 text-red-800",
        BadgeVariant::Neutral => "bg-gray-100 text-gray-800",
    };
    
    view! {
        <span class=format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {}", classes)>
            {children()}
        </span>
    }
}

#[derive(Default, Clone, Copy)]
pub enum BadgeVariant {
    #[default]
    Primary,
    Success,
    Warning,
    Danger,
    Neutral,
}

#[component]
pub fn Alert(
    children: Children,
    #[prop(optional)] variant: AlertVariant,
    #[prop(optional)] dismissible: bool,
    #[prop(optional)] on_dismiss: Option<Callback<()>>,
) -> impl IntoView {
    let (show, set_show) = create_signal(true);
    
    let bg_class = match variant {
        AlertVariant::Info => "bg-blue-50 border-blue-200",
        AlertVariant::Success => "bg-green-50 border-green-200",
        AlertVariant::Warning => "bg-yellow-50 border-yellow-200",
        AlertVariant::Error => "bg-red-50 border-red-200",
    }.to_string();
    
    let text_class = match variant {
        AlertVariant::Info => "text-blue-800",
        AlertVariant::Success => "text-green-800",
        AlertVariant::Warning => "text-yellow-800",
        AlertVariant::Error => "text-red-800",
    };
    
    let icon = match variant {
        AlertVariant::Info => "ℹ️",
        AlertVariant::Success => "✅",
        AlertVariant::Warning => "⚠️",
        AlertVariant::Error => "❌",
    };
    
    view! {
        <Show when=move || show.get()>
            <div class=format!("rounded-md p-4 border {}", bg_class)>
                <div class="flex">
                    <div class="flex-shrink-0">
                        <span class="text-lg">{icon}</span>
                    </div>
                    <div class="ml-3 flex-1">
                        <p class=format!("text-sm {}", text_class)>
                            {children()}
                        </p>
                    </div>
                    <Show when=move || dismissible>
                        <div class="ml-auto pl-3">
                            <button
                                class=format!("-mx-1.5 -my-1.5 rounded-md p-1.5 inline-flex hover:bg-opacity-20 focus:outline-none {}", text_class)
                                on:click=move |_| {
                                    set_show.set(false);
                                    if let Some(callback) = &on_dismiss {
                                        callback.call(());
                                    }
                                }
                            >
                                <span class="sr-only">"Dismiss"</span>
                                <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
                                    <path
                                        fill-rule="evenodd"
                                        d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                                        clip-rule="evenodd"
                                    />
                                </svg>
                            </button>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}

#[derive(Default, Clone, Copy)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}