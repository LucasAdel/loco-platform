use leptos::*;
use wasm_bindgen::prelude::*;
// use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = mapboxgl)]
    type Map;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    fn new(options: &JsValue) -> Map;
}

#[component]
pub fn Map() -> impl IntoView {
    let map_container = create_node_ref::<html::Div>();
    
    create_effect(move |_| {
        if let Some(container) = map_container.get_untracked() {
            // Initialize Mapbox
            let options = js_sys::Object::new();
            js_sys::Reflect::set(
                &options,
                &"container".into(),
                &container.clone().unchecked_into::<wasm_bindgen::JsValue>(),
            ).unwrap();
            js_sys::Reflect::set(
                &options,
                &"style".into(),
                &"mapbox://styles/mapbox/streets-v11".into(),
            ).unwrap();
            js_sys::Reflect::set(
                &options,
                &"center".into(),
                &js_sys::Array::of2(&JsValue::from(133.7751), &JsValue::from(-25.2744)),
            ).unwrap();
            js_sys::Reflect::set(
                &options,
                &"zoom".into(),
                &JsValue::from(4),
            ).unwrap();
            
            // Create map instance
            let _map = Map::new(&options);
            
            // TODO: Add job markers
        }
    });

    view! {
        <div class="h-screen relative">
            // Map container
            <div
                node_ref=map_container
                class="absolute inset-0"
                id="map"
            />
            
            // Overlay controls
            <div class="absolute top-4 left-4 right-4 z-10">
                <div class="bg-white/90 backdrop-blur-md rounded-lg shadow-lg p-4">
                    <h2 class="text-xl font-semibold mb-4">"Job Locations"</h2>
                    
                    // Filter controls
                    <div class="flex flex-wrap gap-4">
                        <select class="px-3 py-2 border border-gray-300 rounded-md">
                            <option>"All Job Types"</option>
                            <option>"Full-time"</option>
                            <option>"Part-time"</option>
                            <option>"Contract"</option>
                            <option>"Casual"</option>
                        </select>
                        
                        <button class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
                            "My Location"
                        </button>
                    </div>
                </div>
            </div>
            
            // Job list sidebar
            <div class="absolute top-24 left-4 bottom-4 w-96 bg-white/90 backdrop-blur-md rounded-lg shadow-lg p-4 overflow-y-auto">
                <h3 class="font-semibold mb-4">"Nearby Jobs"</h3>
                <div class="space-y-3">
                    <p class="text-gray-600">"Loading job locations..."</p>
                </div>
            </div>
        </div>
    }
}