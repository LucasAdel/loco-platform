use leptos::*;
use leptos::prelude::*;

/// Skeleton loading component for placeholder content
#[component]
pub fn Skeleton(
    /// Width of the skeleton (CSS value)
    #[prop(default = "100%")]
    width: &'static str,
    
    /// Height of the skeleton (CSS value)
    #[prop(default = "1rem")]
    height: &'static str,
    
    /// Whether to use circular shape
    #[prop(default = false)]
    circle: bool,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let shape_class = if circle { "rounded-full" } else { "rounded-md" };
    let custom_class = class.unwrap_or("");
    
    view! {
        <div
            class=format!(
                "animate-pulse bg-gray-300 dark:bg-gray-700 {} {}",
                shape_class,
                custom_class
            )
            style=format!("width: {}; height: {};", width, height)
        />
    }
}

/// Skeleton text component that mimics text layout
#[component]
pub fn SkeletonText(
    /// Number of lines
    #[prop(default = 3)]
    lines: usize,
    
    /// Whether to vary line widths
    #[prop(default = true)]
    vary_widths: bool,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let widths = if vary_widths {
        vec!["100%", "85%", "75%", "90%", "80%", "95%"]
    } else {
        vec!["100%"; 6]
    };
    
    view! {
        <div class=format!("space-y-2 {}", class.unwrap_or(""))>
            {(0..lines).map(|i| {
                let width = widths.get(i % widths.len()).unwrap_or(&"100%");
                view! {
                    <Skeleton width=width height="0.875rem" />
                }
            }).collect_view()}
        </div>
    }
}

/// Skeleton card component
#[component]
pub fn SkeletonCard(
    /// Whether to show image skeleton
    #[prop(default = false)]
    with_image: bool,
    
    /// Whether to show avatar
    #[prop(default = false)]
    with_avatar: bool,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class=format!(
            "bg-primary border border-primary rounded-xl p-6 {}",
            class.unwrap_or("")
        )>
            {if with_image {
                view! {
                    <div class="mb-4">
                        <Skeleton width="100%" height="200px" class="rounded-lg" />
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
            
            {if with_avatar {
                view! {
                    <div class="flex items-start space-x-4 mb-4">
                        <Skeleton width="3rem" height="3rem" circle=true />
                        <div class="flex-1">
                            <Skeleton width="150px" height="1.25rem" class="mb-2" />
                            <Skeleton width="100px" height="0.875rem" />
                        </div>
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
            
            <SkeletonText lines=3 />
            
            <div class="mt-4 flex space-x-2">
                <Skeleton width="80px" height="32px" class="rounded-lg" />
                <Skeleton width="80px" height="32px" class="rounded-lg" />
            </div>
        </div>
    }
}

/// Skeleton list component
#[component]
pub fn SkeletonList(
    /// Number of items to show
    #[prop(default = 5)]
    items: usize,
    
    /// Whether to show avatars
    #[prop(default = false)]
    with_avatars: bool,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class=format!("space-y-4 {}", class.unwrap_or(""))>
            {(0..items).map(|_| view! {
                <div class="flex items-center space-x-4">
                    {if with_avatars {
                        view! {
                            <Skeleton width="2.5rem" height="2.5rem" circle=true />
                        }
                    } else {
                        view! { <div></div> }
                    }}
                    <div class="flex-1">
                        <Skeleton width="60%" height="1rem" class="mb-2" />
                        <Skeleton width="40%" height="0.875rem" />
                    </div>
                    <Skeleton width="60px" height="24px" class="rounded-full" />
                </div>
            }).collect_view()}
        </div>
    }
}

/// Skeleton table component
#[component]
pub fn SkeletonTable(
    /// Number of rows
    #[prop(default = 5)]
    rows: usize,
    
    /// Number of columns
    #[prop(default = 4)]
    columns: usize,
    
    /// Whether to show header
    #[prop(default = true)]
    with_header: bool,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class=format!(
            "bg-primary border border-primary rounded-lg overflow-hidden {}",
            class.unwrap_or("")
        )>
            <table class="w-full">
                {if with_header {
                    view! {
                        <thead class="bg-secondary">
                            <tr>
                                {(0..columns).map(|_| view! {
                                    <th class="px-6 py-3">
                                        <Skeleton width="80%" height="0.875rem" />
                                    </th>
                                }).collect_view()}
                            </tr>
                        </thead>
                    }
                } else {
                    view! { <thead></thead> }
                }}
                <tbody>
                    {(0..rows).map(|_| view! {
                        <tr class="border-t border-primary">
                            {(0..columns).map(|col| {
                                let width = match col {
                                    0 => "70%",
                                    _ => "50%",
                                };
                                view! {
                                    <td class="px-6 py-4">
                                        <Skeleton width=width height="0.875rem" />
                                    </td>
                                }
                            }).collect_view()}
                        </tr>
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
}

/// Loading spinner component
#[component]
pub fn LoadingSpinner(
    /// Size of the spinner
    #[prop(default = SpinnerSize::Medium)]
    size: SpinnerSize,
    
    /// Colour of the spinner
    #[prop(default = "text-blue-600")]
    color: &'static str,
    
    /// Additional CSS classes
    #[prop(optional)]
    class: Option<&'static str>,
) -> impl IntoView {
    let size_class = match size {
        SpinnerSize::Small => "w-4 h-4",
        SpinnerSize::Medium => "w-8 h-8",
        SpinnerSize::Large => "w-12 h-12",
    };
    
    view! {
        <div class=format!("inline-block {}", class.unwrap_or(""))>
            <div class=format!(
                "animate-spin rounded-full border-2 border-solid border-current border-r-transparent {} {}",
                size_class,
                color
            )>
                <span class="sr-only">Loading...</span>
            </div>
        </div>
    }
}

/// Spinner size variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerSize {
    Small,
    Medium,
    Large,
}

/// Loading overlay component
#[component]
pub fn LoadingOverlay(
    /// Whether to show the overlay
    show: ReadSignal<bool>,
    
    /// Loading message
    #[prop(optional)]
    message: Option<&'static str>,
    
    /// Whether to blur background
    #[prop(default = true)]
    blur_background: bool,
) -> impl IntoView {
    view! {
        <Show when=show>
            <div class=format!(
                "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 {}",
                if blur_background { "backdrop-blur-sm" } else { "" }
            )>
                <div class="bg-primary rounded-lg p-6 shadow-xl flex flex-col items-center">
                    <LoadingSpinner size=SpinnerSize::Large />
                    {message.map(|msg| view! {
                        <p class="mt-4 text-primary font-medium">{msg}</p>
                    })}
                </div>
            </div>
        </Show>
    }
}

/// Content loader wrapper that shows skeleton while loading
#[component]
pub fn ContentLoader<F, V>(
    /// Loading state
    loading: ReadSignal<bool>,
    
    /// Skeleton component to show while loading
    skeleton: F,
    
    /// Content to show when loaded
    children: Children,
) -> impl IntoView
where
    F: Fn() -> V + 'static,
    V: IntoView,
{
    view! {
        <Show
            when=loading
            fallback=children
        >
            {skeleton()}
        </Show>
    }
}