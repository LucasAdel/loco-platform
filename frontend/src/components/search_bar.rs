use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchBarProps {
    on_search: EventHandler<String>,
}

#[component]
pub fn SearchBar(props: SearchBarProps) -> Element {
    let mut search_value = use_signal(|| String::new());
    
    rsx! {
        div {
            class: "relative w-full max-w-2xl",
            
            // Search input
            input {
                class: "w-full px-4 py-3 pl-12 pr-4 text-gray-900 bg-white border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent shadow-sm",
                r#type: "search",
                "aria-label": "Search for jobs by address, suburb or postcode",
                placeholder: "Search for address, suburb or postcode...",
                value: "{search_value()}",
                oninput: move |event| {
                    let value = event.value();
                    search_value.set(value.clone());
                    props.on_search.call(value);
                }
            }
            
            // Search icon
            div {
                class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                svg {
                    class: "h-5 w-5 text-gray-400",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round", 
                        stroke_width: "2",
                        d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    }
                }
            }
        }
    }
}