use leptos::*;
use leptos::prelude::*;
use std::rc::Rc;

/// Error boundary component that catches and displays errors gracefully
#[component]
pub fn ErrorBoundary(
    children: Children,
    /// Optional fallback component to show when an error occurs
    #[prop(optional)]
    fallback: Option<Rc<dyn Fn(String) -> View>>,
) -> impl IntoView {
    let (error_msg, set_error_msg) = create_signal(None::<String>);
    
    // Create error handler
    let error_handler = Rc::new(move |err_msg: String| {
        leptos::logging::error!("Error boundary caught: {}", err_msg);
        set_error_msg.set(Some(err_msg));
    });
    
    // Provide error handler context
    provide_context(error_handler.clone());
    
    view! {
        <Show
            when=move || error_msg.get().is_some()
            fallback=|| children()
        >
            {move || {
                let msg = error_msg.get().unwrap_or_default();
                if let Some(custom_fallback) = &fallback {
                    custom_fallback(msg)
                } else {
                    // Default error display
                    view! {
                        <DefaultErrorDisplay message=msg.clone() on_retry=Box::new(move || set_error_msg.set(None)) />
                    }.into_view()
                }
            }}
        </Show>
    }
}

/// Default error display component
#[component]
fn DefaultErrorDisplay(
    message: String,
    on_retry: Box<dyn Fn()>,
) -> impl IntoView {
    view! {
        <div class="min-h-[400px] flex items-center justify-center p-4">
            <div class="bg-red-50 border border-red-200 rounded-lg p-6 max-w-md w-full">
                <div class="flex items-start">
                    <div class="flex-shrink-0">
                        <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>
                    <div class="ml-3 flex-1">
                        <h3 class="text-lg font-medium text-red-800">
                            "Something went wrong"
                        </h3>
                        <div class="mt-2 text-sm text-red-700">
                            <p>{message}</p>
                        </div>
                        <div class="mt-4">
                            <button
                                on:click=move |_| on_retry()
                                class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-red-700 bg-red-100 hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 transition-base"
                            >
                                "Try Again"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Hook to use the error handler from ErrorBoundary context
pub fn use_error_handler() -> Rc<dyn Fn(String)> {
    use_context::<Rc<dyn Fn(String)>>()
        .unwrap_or_else(|| {
            // Fallback error handler if no ErrorBoundary is present
            Rc::new(move |err_msg: String| {
                leptos::logging::error!("Unhandled error: {}", err_msg);
            })
        })
}

/// Macro to easily wrap components with error boundary
#[macro_export]
macro_rules! with_error_boundary {
    ($component:expr) => {
        move || {
            view! {
                <ErrorBoundary>
                    {$component}
                </ErrorBoundary>
            }
        }
    };
    ($component:expr, $fallback:expr) => {
        move || {
            view! {
                <ErrorBoundary fallback=$fallback>
                    {$component}
                </ErrorBoundary>
            }
        }
    };
}