use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use js_sys::Array;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JobMarker {
    id: String,
    title: String,
    suburb: String,
    pharmacy_name: String,
    hourly_rate: f64,
    latitude: f64,
    longitude: f64,
    is_urgent: bool,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = mapboxgl)]
    type Map;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    fn new(options: &JsValue) -> Map;
    
    #[wasm_bindgen(method, js_namespace = mapboxgl)]
    fn on(this: &Map, event: &str, callback: &js_sys::Function);
    
    #[wasm_bindgen(method, js_namespace = mapboxgl)]
    fn addSource(this: &Map, id: &str, source: &JsValue);
    
    #[wasm_bindgen(method, js_namespace = mapboxgl)]
    fn addLayer(this: &Map, layer: &JsValue);

    #[wasm_bindgen(js_namespace = mapboxgl)]
    type Marker;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    fn new_marker(options: &JsValue) -> Marker;
    
    #[wasm_bindgen(method)]
    fn setLngLat(this: &Marker, coords: &Array) -> &Marker;
    
    #[wasm_bindgen(method)]
    fn setPopup(this: &Marker, popup: &Popup) -> &Marker;
    
    #[wasm_bindgen(method)]
    fn addTo(this: &Marker, map: &Map) -> &Marker;

    #[wasm_bindgen(js_namespace = mapboxgl)]
    type Popup;

    #[wasm_bindgen(constructor, js_namespace = mapboxgl)]
    fn new_popup(options: &JsValue) -> Popup;
    
    #[wasm_bindgen(method)]
    fn setHTML(this: &Popup, html: &str) -> &Popup;
}

#[component]
pub fn MapEnhanced() -> impl IntoView {
    let map_container = create_node_ref::<HtmlElement>();
    let (selected_suburb, set_selected_suburb) = create_signal(String::from("All"));
    let (job_type_filter, set_job_type_filter) = create_signal(String::from("All"));
    let (jobs_data, set_jobs_data) = create_signal(Vec::<JobMarker>::new());
    
    // Fetch jobs data
    spawn_local(async move {
        // In production, this would fetch from your API
        // For now, using mock data that reflects the suburb distribution
        let mock_jobs = vec![
            JobMarker {
                id: "1".to_string(),
                title: "Senior Pharmacist".to_string(),
                suburb: "Norwood".to_string(),
                pharmacy_name: "Norwood Pharmacy".to_string(),
                hourly_rate: 55.0,
                latitude: -34.9206,
                longitude: 138.6326,
                is_urgent: false,
            },
            JobMarker {
                id: "2".to_string(),
                title: "Locum Pharmacist".to_string(),
                suburb: "Burnside".to_string(),
                pharmacy_name: "Burnside Medical".to_string(),
                hourly_rate: 65.0,
                latitude: -34.9397,
                longitude: 138.6444,
                is_urgent: true,
            },
            JobMarker {
                id: "3".to_string(),
                title: "Part-time Pharmacist".to_string(),
                suburb: "Glenelg".to_string(),
                pharmacy_name: "Beach Pharmacy".to_string(),
                hourly_rate: 50.0,
                latitude: -34.9823,
                longitude: 138.5166,
                is_urgent: false,
            },
            // Add more jobs here based on actual data
        ];
        
        set_jobs_data.set(mock_jobs);
    });
    
    create_effect(move |_| {
        if let Some(container) = map_container.get() {
            // Initialize Mapbox centered on Adelaide
            let options = js_sys::Object::new();
            js_sys::Reflect::set(
                &options,
                &"container".into(),
                &container.into(),
            ).unwrap();
            js_sys::Reflect::set(
                &options,
                &"style".into(),
                &"mapbox://styles/mapbox/light-v11".into(),
            ).unwrap();
            js_sys::Reflect::set(
                &options,
                &"center".into(),
                &js_sys::Array::of2(&JsValue::from(138.6007), &JsValue::from(-34.9285)),
            ).unwrap();
            js_sys::Reflect::set(
                &options,
                &"zoom".into(),
                &JsValue::from(10),
            ).unwrap();
            
            // Create map instance
            let map = Map::new(&options);
            
            // Wait for map to load
            let closure = Closure::wrap(Box::new(move || {
                // Add markers for each job
                for job in jobs_data.get() {
                    // Create marker options
                    let marker_options = js_sys::Object::new();
                    js_sys::Reflect::set(
                        &marker_options,
                        &"color".into(),
                        &(if job.is_urgent { "#ef4444" } else { "#3b82f6" }).into(),
                    ).unwrap();
                    
                    // Create popup content
                    let popup_html = format!(
                        r#"<div class="p-3">
                            <h3 class="font-semibold text-lg">{}</h3>
                            <p class="text-gray-600">{}</p>
                            <p class="text-gray-700">{}</p>
                            <p class="font-medium mt-2">${:.2}/hr</p>
                            {}
                        </div>"#,
                        job.title,
                        job.pharmacy_name,
                        job.suburb,
                        job.hourly_rate,
                        if job.is_urgent { "<span class='text-red-600 font-bold'>URGENT</span>" } else { "" }
                    );
                    
                    // Create popup
                    let popup_options = js_sys::Object::new();
                    js_sys::Reflect::set(
                        &popup_options,
                        &"offset".into(),
                        &JsValue::from(25),
                    ).unwrap();
                    
                    let popup = Popup::new_popup(&popup_options);
                    popup.setHTML(&popup_html);
                    
                    // Create and add marker
                    let marker = Marker::new_marker(&marker_options);
                    marker
                        .setLngLat(&js_sys::Array::of2(&JsValue::from(job.longitude), &JsValue::from(job.latitude)))
                        .setPopup(&popup)
                        .addTo(&map);
                }
            }) as Box<dyn FnMut()>);
            
            let js_func = closure.as_ref().unchecked_ref::<js_sys::Function>();
            map.on("load", js_func);
            closure.forget(); // Keep closure alive
        }
    });
    
    // Get unique suburbs for filter
    let suburbs = create_memo(move |_| {
        let mut suburbs: Vec<String> = jobs_data.get()
            .iter()
            .map(|job| job.suburb.clone())
            .collect();
        suburbs.sort();
        suburbs.dedup();
        suburbs
    });

    view! {
        <div class="h-screen relative">
            // Map container
            <div
                node_ref=map_container
                class="absolute inset-0"
                id="map"
            />
            
            // Top control bar
            <div class="absolute top-4 left-4 right-4 z-10 pointer-events-none">
                <div class="bg-white/95 backdrop-blur-md rounded-xl shadow-lg p-4 pointer-events-auto max-w-4xl mx-auto">
                    <h2 class="text-2xl font-bold mb-4 text-gray-800">"Pharmacy Jobs Map"</h2>
                    
                    // Filter controls
                    <div class="flex flex-wrap gap-4">
                        <select 
                            class="px-4 py-2 border border-gray-300 rounded-lg bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            on:change=move |ev| {
                                set_selected_suburb.set(event_target_value(&ev));
                            }
                        >
                            <option value="All">"All Suburbs"</option>
                            {suburbs.get().into_iter().map(|suburb| {
                                view! {
                                    <option value=suburb.clone()>{suburb}</option>
                                }
                            }).collect_view()}
                        </select>
                        
                        <select 
                            class="px-4 py-2 border border-gray-300 rounded-lg bg-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            on:change=move |ev| {
                                set_job_type_filter.set(event_target_value(&ev));
                            }
                        >
                            <option value="All">"All Job Types"</option>
                            <option value="Full-time">"Full-time"</option>
                            <option value="Part-time">"Part-time"</option>
                            <option value="Contract">"Contract"</option>
                            <option value="Casual">"Casual"</option>
                        </select>
                        
                        <button class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium">
                            <i class="fas fa-location-arrow mr-2"></i>
                            "My Location"
                        </button>
                        
                        <div class="ml-auto flex items-center gap-4">
                            <span class="flex items-center gap-2">
                                <span class="w-4 h-4 bg-blue-500 rounded-full"></span>
                                <span class="text-sm text-gray-600">"Regular"</span>
                            </span>
                            <span class="flex items-center gap-2">
                                <span class="w-4 h-4 bg-red-500 rounded-full"></span>
                                <span class="text-sm text-gray-600">"Urgent"</span>
                            </span>
                        </div>
                    </div>
                </div>
            </div>
            
            // Job list sidebar
            <div class="absolute top-32 left-4 bottom-4 w-96 bg-white/95 backdrop-blur-md rounded-xl shadow-lg overflow-hidden pointer-events-auto">
                <div class="p-4 border-b bg-gray-50">
                    <h3 class="font-bold text-lg text-gray-800">"Available Positions"</h3>
                    <p class="text-sm text-gray-600 mt-1">
                        {move || format!("{} jobs found", jobs_data.get().len())}
                    </p>
                </div>
                
                <div class="overflow-y-auto h-full pb-20">
                    <div class="p-4 space-y-3">
                        {move || jobs_data.get().into_iter()
                            .filter(|job| {
                                selected_suburb.get() == "All" || job.suburb == selected_suburb.get()
                            })
                            .map(|job| {
                                view! {
                                    <div class="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow cursor-pointer">
                                        <div class="flex justify-between items-start">
                                            <div>
                                                <h4 class="font-semibold text-gray-800">{&job.title}</h4>
                                                <p class="text-sm text-gray-600 mt-1">{&job.pharmacy_name}</p>
                                                <p class="text-sm text-gray-500 mt-1">
                                                    <i class="fas fa-map-marker-alt mr-1"></i>
                                                    {&job.suburb}
                                                </p>
                                            </div>
                                            <div class="text-right">
                                                <p class="font-bold text-lg text-gray-800">
                                                    {format!("${:.0}/hr", job.hourly_rate)}
                                                </p>
                                                {if job.is_urgent {
                                                    view! {
                                                        <span class="inline-block mt-1 px-2 py-1 bg-red-100 text-red-700 text-xs font-bold rounded">
                                                            "URGENT"
                                                        </span>
                                                    }
                                                } else {
                                                    view! { <span></span> }
                                                }}
                                            </div>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()
                        }
                    </div>
                </div>
            </div>
            
            // Map legend
            <div class="absolute bottom-4 right-4 bg-white/95 backdrop-blur-md rounded-lg shadow-lg p-4">
                <h4 class="font-semibold text-sm mb-2">"Adelaide Metro Area"</h4>
                <p class="text-xs text-gray-600">"Zoom and pan to explore"</p>
            </div>
        </div>
    }
}