use leptos::*;
use leptos::prelude::*;
use leptos::html::Input;

#[component]
pub fn SearchBar(
    #[prop(into)] on_search: Callback<String>,
    #[prop(default = String::new())] placeholder: String,
) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let (search_value, set_search_value) = create_signal(String::new());

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        on_search.call(search_value.get());
    };

    let handle_input = move |ev| {
        let value = event_target_value(&ev);
        set_search_value.set(value.clone());
        on_search.call(value);
    };

    view! {
        <form on:submit=handle_submit class="w-full">
            <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                    <svg class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                        />
                    </svg>
                </div>
                <input
                    node_ref=input_ref
                    type="text"
                    class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                    placeholder=placeholder
                    value=search_value
                    on:input=handle_input
                />
            </div>
        </form>
    }
}