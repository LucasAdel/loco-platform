use leptos::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::HtmlElement;

/// Modal size variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    FullWidth,
}

impl ModalSize {
    fn class(&self) -> &'static str {
        match self {
            ModalSize::Small => "max-w-md",
            ModalSize::Medium => "max-w-2xl",
            ModalSize::Large => "max-w-4xl",
            ModalSize::FullWidth => "max-w-7xl",
        }
    }
}

/// Modal component with portal rendering
#[component]
pub fn Modal(
    /// Whether the modal is open
    show: ReadSignal<bool>,
    
    /// Callback to close the modal
    on_close: impl Fn() + 'static,
    
    /// Modal title
    #[prop(optional)]
    title: Option<&'static str>,
    
    /// Modal size
    #[prop(default = ModalSize::Medium)]
    size: ModalSize,
    
    /// Whether to show close button
    #[prop(default = true)]
    show_close_button: bool,
    
    /// Whether clicking backdrop closes modal
    #[prop(default = true)]
    close_on_backdrop_click: bool,
    
    /// Whether pressing Escape closes modal
    #[prop(default = true)]
    close_on_escape: bool,
    
    /// Footer content
    #[prop(optional)]
    footer: Option<View>,
    
    /// Modal content
    children: Children,
) -> impl IntoView {
    let modal_ref = create_node_ref::<HtmlElement>();
    
    // Handle escape key
    create_effect(move |_| {
        if show.get() && close_on_escape {
            let on_close = on_close.clone();
            let handle_keydown = move |ev: web_sys::KeyboardEvent| {
                if ev.key() == "Escape" {
                    on_close();
                }
            };
            
            if let Some(window) = web_sys::window() {
                let closure = Closure::wrap(Box::new(handle_keydown) as Box<dyn FnMut(_)>);
                let _ = window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                closure.forget();
            }
        }
    });
    
    // Handle backdrop click
    let handle_backdrop_click = move |ev: web_sys::MouseEvent| {
        if close_on_backdrop_click {
            if let Some(modal) = modal_ref.get() {
                let target = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
                if !modal.contains(Some(&target)) {
                    on_close();
                }
            }
        }
    };
    
    // Portal the modal to body
    let portal = create_portal(move || {
        view! {
            <Show when=move || show.get()>
                <div 
                    class="fixed inset-0 z-50 overflow-y-auto"
                    on:click=handle_backdrop_click
                >
                    // Backdrop
                    <div class="fixed inset-0 bg-black bg-opacity-50 transition-opacity" />
                    
                    // Modal container
                    <div class="flex min-h-full items-center justify-center p-4">
                        <div
                            node_ref=modal_ref
                            class=format!(
                                "relative w-full {} transform overflow-hidden rounded-lg bg-primary shadow-xl transition-all",
                                size.class()
                            )
                        >
                            // Header
                            {title.map(|t| view! {
                                <div class="border-b border-primary px-6 py-4">
                                    <div class="flex items-center justify-between">
                                        <h3 class="text-lg font-semibold text-primary">
                                            {t}
                                        </h3>
                                        {if show_close_button {
                                            view! {
                                                <button
                                                    on:click=move |_| on_close()
                                                    class="rounded-lg p-1 text-secondary hover:bg-secondary transition-base"
                                                    aria-label="Close modal"
                                                >
                                                    <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                                    </svg>
                                                </button>
                                            }
                                        } else {
                                            view! { <span></span> }
                                        }}
                                    </div>
                                </div>
                            })}
                            
                            // Content
                            <div class="px-6 py-4">
                                {children()}
                            </div>
                            
                            // Footer
                            {footer.map(|f| view! {
                                <div class="border-t border-primary px-6 py-4">
                                    {f}
                                </div>
                            })}
                        </div>
                    </div>
                </div>
            </Show>
        }
    });
    
    portal
}

/// Helper function to create a portal
fn create_portal<F, V>(child: F) -> impl IntoView
where
    F: Fn() -> V + 'static,
    V: IntoView,
{
    let window = web_sys::window().expect("Window should exist");
    let document = window.document().expect("Document should exist");
    let body = document.body().expect("Body should exist");
    
    // Create portal container
    let portal_id = "modal-portal";
    let portal_container = if let Some(existing) = document.get_element_by_id(portal_id) {
        existing
    } else {
        let container = document.create_element("div").expect("Should create div");
        container.set_id(portal_id);
        body.append_child(&container).expect("Should append container");
        container
    };
    
    // Mount the content
    mount_to(portal_container.unchecked_into(), child)
}

/// Confirmation modal component
#[component]
pub fn ConfirmModal(
    /// Whether the modal is open
    show: ReadSignal<bool>,
    
    /// Modal title
    #[prop(default = "Confirm Action")]
    title: &'static str,
    
    /// Confirmation message
    message: &'static str,
    
    /// Confirm button text
    #[prop(default = "Confirm")]
    confirm_text: &'static str,
    
    /// Cancel button text
    #[prop(default = "Cancel")]
    cancel_text: &'static str,
    
    /// Whether the action is dangerous (changes button color)
    #[prop(default = false)]
    is_dangerous: bool,
    
    /// Callback when confirmed
    on_confirm: impl Fn() + 'static + Clone,
    
    /// Callback when cancelled
    on_cancel: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let on_confirm = on_confirm.clone();
    let on_cancel = on_cancel.clone();
    
    let footer = view! {
        <div class="flex justify-end gap-3">
            <button
                on:click=move |_| on_cancel()
                class="px-4 py-2 text-sm font-medium text-secondary bg-secondary hover:bg-tertiary rounded-lg transition-base"
            >
                {cancel_text}
            </button>
            <button
                on:click=move |_| on_confirm()
                class=format!(
                    "px-4 py-2 text-sm font-medium text-white rounded-lg transition-base {}",
                    if is_dangerous {
                        "bg-red-600 hover:bg-red-700"
                    } else {
                        "bg-blue-600 hover:bg-blue-700"
                    }
                )
            >
                {confirm_text}
            </button>
        </div>
    };
    
    view! {
        <Modal
            show=show
            on_close=on_cancel
            title=Some(title)
            size=ModalSize::Small
            footer=Some(footer.into_view())
        >
            <p class="text-secondary">{message}</p>
        </Modal>
    }
}

/// Alert modal component
#[component]
pub fn AlertModal(
    /// Whether the modal is open
    show: ReadSignal<bool>,
    
    /// Callback to close the modal
    on_close: impl Fn() + 'static + Clone,
    
    /// Alert type
    #[prop(default = AlertType::Info)]
    alert_type: AlertType,
    
    /// Alert title
    title: &'static str,
    
    /// Alert message
    message: &'static str,
    
    /// Button text
    #[prop(default = "OK")]
    button_text: &'static str,
) -> impl IntoView {
    let icon = match alert_type {
        AlertType::Success => view! {
            <svg class="h-6 w-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
        },
        AlertType::Error => view! {
            <svg class="h-6 w-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
        },
        AlertType::Warning => view! {
            <svg class="h-6 w-6 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
        },
        AlertType::Info => view! {
            <svg class="h-6 w-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
        },
    };
    
    let footer = view! {
        <div class="flex justify-center">
            <button
                on:click=move |_| on_close()
                class="px-6 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg transition-base"
            >
                {button_text}
            </button>
        </div>
    };
    
    view! {
        <Modal
            show=show
            on_close=on_close
            size=ModalSize::Small
            show_close_button=false
            footer=Some(footer.into_view())
        >
            <div class="flex items-start gap-4">
                <div class="flex-shrink-0">
                    {icon}
                </div>
                <div>
                    <h4 class="text-lg font-semibold text-primary mb-1">{title}</h4>
                    <p class="text-secondary">{message}</p>
                </div>
            </div>
        </Modal>
    }
}

/// Alert types for AlertModal
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertType {
    Success,
    Error,
    Warning,
    Info,
}