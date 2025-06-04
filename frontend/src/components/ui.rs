use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = "primary".to_string())]
    variant: String,
    #[props(default = "md".to_string())]
    size: String,
    #[props(default = false)]
    disabled: bool,
    #[props(optional)]
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_classes = "inline-flex items-center justify-center font-medium rounded-lg transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2";
    
    let variant_classes = match props.variant.as_str() {
        "primary" => "bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500",
        "secondary" => "bg-gray-100 text-gray-900 hover:bg-gray-200 focus:ring-gray-500",
        "success" => "bg-green-600 text-white hover:bg-green-700 focus:ring-green-500",
        "danger" => "bg-red-600 text-white hover:bg-red-700 focus:ring-red-500",
        "outline" => "border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:ring-blue-500",
        _ => "bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500",
    };
    
    let size_classes = match props.size.as_str() {
        "sm" => "px-3 py-2 text-sm",
        "md" => "px-4 py-2 text-sm",
        "lg" => "px-6 py-3 text-base",
        _ => "px-4 py-2 text-sm",
    };
    
    let disabled_classes = if props.disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        ""
    };
    
    rsx! {
        button {
            class: "{base_classes} {variant_classes} {size_classes} {disabled_classes}",
            disabled: props.disabled,
            onclick: move |e| {
                if let Some(handler) = &props.onclick {
                    handler.call(e);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    #[props(default = "text".to_string())]
    r#type: String,
    #[props(optional)]
    placeholder: Option<String>,
    #[props(optional)]
    value: Option<String>,
    #[props(optional)]
    oninput: Option<EventHandler<FormEvent>>,
    #[props(default = false)]
    disabled: bool,
    #[props(optional)]
    class: Option<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let base_classes = "block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-blue-500 focus:border-blue-500";
    let disabled_classes = if props.disabled {
        "bg-gray-100 cursor-not-allowed"
    } else {
        ""
    };
    let custom_classes = props.class.unwrap_or_default();
    
    rsx! {
        input {
            r#type: "{props.r#type}",
            class: "{base_classes} {disabled_classes} {custom_classes}",
            placeholder: props.placeholder.unwrap_or_default(),
            value: props.value.unwrap_or_default(),
            disabled: props.disabled,
            oninput: move |e| {
                if let Some(handler) = &props.oninput {
                    handler.call(e);
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(optional)]
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let base_classes = "bg-white shadow rounded-lg";
    let custom_classes = props.class.unwrap_or_default();
    
    rsx! {
        div {
            class: "{base_classes} {custom_classes}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    show: bool,
    #[props(optional)]
    onclose: Option<EventHandler<()>>,
    children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !props.show {
        return rsx! { div {} };
    }
    
    rsx! {
        div {
            class: "fixed inset-0 z-50 overflow-y-auto",
            
            // Backdrop
            div {
                class: "fixed inset-0 bg-black bg-opacity-50",
                onclick: move |_| {
                    if let Some(handler) = &props.onclose {
                        handler.call(());
                    }
                }
            }
            
            // Modal content
            div {
                class: "relative min-h-screen flex items-center justify-center p-4",
                div {
                    class: "relative bg-white rounded-lg shadow-lg max-w-md w-full",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    {props.children}
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct LoadingSpinnerProps {
    #[props(default = "md".to_string())]
    size: String,
}

#[component]
pub fn LoadingSpinner(props: LoadingSpinnerProps) -> Element {
    let size_classes = match props.size.as_str() {
        "sm" => "w-4 h-4",
        "md" => "w-8 h-8",
        "lg" => "w-12 h-12",
        _ => "w-8 h-8",
    };
    
    rsx! {
        div {
            class: "flex justify-center items-center",
            div {
                class: "animate-spin rounded-full border-2 border-gray-300 border-t-blue-600 {size_classes}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default = "gray".to_string())]
    variant: String,
    children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let variant_classes = match props.variant.as_str() {
        "gray" => "bg-gray-100 text-gray-800",
        "blue" => "bg-blue-100 text-blue-800",
        "green" => "bg-green-100 text-green-800",
        "yellow" => "bg-yellow-100 text-yellow-800",
        "red" => "bg-red-100 text-red-800",
        _ => "bg-gray-100 text-gray-800",
    };
    
    rsx! {
        span {
            class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {variant_classes}",
            {props.children}
        }
    }
}