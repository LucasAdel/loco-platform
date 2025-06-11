use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Mapbox configuration
const MAPBOX_TOKEN: &str = "pk.eyJ1IjoiaGVhbHRocGFnZXMiLCJhIjoiY204cGduaWxxMGF0cDJxcG5jeG03ZXRheiJ9.zfIAdS9mexKP1RNSDEI4Og";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLocation {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub description: Option<String>,
    pub is_urgent: bool,
    pub job_type: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct MapCenter {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: f64,
}

impl Default for MapCenter {
    fn default() -> Self {
        // Default to Australia center
        Self {
            latitude: -25.2744,
            longitude: 133.7751,
            zoom: 4.0,
        }
    }
}

// JavaScript bindings for Mapbox
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn initializeMapbox(
        container_id: &str,
        token: &str,
        center_lat: f64,
        center_lng: f64,
        zoom: f64,
    ) -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    fn addMapboxMarker(
        map: &JsValue,
        id: &str,
        lat: f64,
        lng: f64,
        title: &str,
        description: &str,
        is_urgent: bool,
    );

    #[wasm_bindgen(js_namespace = window)]
    fn clearMapboxMarkers(map: &JsValue);

    #[wasm_bindgen(js_namespace = window)]
    fn updateMapCenter(map: &JsValue, lat: f64, lng: f64, zoom: f64);

    #[wasm_bindgen(js_namespace = window)]
    fn fitMapBounds(map: &JsValue, locations_json: &str);
}

#[component]
pub fn MapboxComponent(
    locations: Signal<Vec<MapLocation>>,
    #[prop(optional)] center: Option<Signal<MapCenter>>,
    #[prop(optional)] on_location_click: Option<Box<dyn Fn(String) + 'static>>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let map_ref = NodeRef::<html::Div>::new();
    let map_instance = RwSignal::new(None::<JsValue>);

    // Initialize map when component mounts
    create_effect(move |_| {
        if let Some(element) = map_ref.get() {
            let container_id = format!("mapbox-{}", uuid::Uuid::new_v4());
            
            // Set the ID on the element
            let _ = element.set_attribute("id", &container_id);
            
            // Wait for next tick to ensure DOM is ready
            set_timeout(
                move || {
                    let center_data = center.map(|c| c.get()).unwrap_or_default();
                    
                    // Initialize Mapbox
                    let map = initializeMapbox(
                        &container_id,
                        MAPBOX_TOKEN,
                        center_data.latitude,
                        center_data.longitude,
                        center_data.zoom,
                    );
                    
                    map_instance.set(Some(map));
                    
                    // Add initial markers
                    let locs = locations.get();
                    if let Some(map) = map_instance.get() {
                        for location in &locs {
                            addMapboxMarker(
                                &map,
                                &location.id,
                                location.latitude,
                                location.longitude,
                                &location.title,
                                location.description.as_deref().unwrap_or(""),
                                location.is_urgent,
                            );
                        }
                        
                        // Fit bounds to show all markers
                        if !locs.is_empty() {
                            let locations_json = serde_json::to_string(&locs).unwrap_or_default();
                            fitMapBounds(&map, &locations_json);
                        }
                    }
                },
                100,
            );
        }
    });

    // Update markers when locations change
    create_effect(move |_| {
        let locs = locations.get();
        
        if let Some(map) = map_instance.get() {
            // Clear existing markers
            clearMapboxMarkers(&map);
            
            // Add new markers
            for location in &locs {
                addMapboxMarker(
                    &map,
                    &location.id,
                    location.latitude,
                    location.longitude,
                    &location.title,
                    location.description.as_deref().unwrap_or(""),
                    location.is_urgent,
                );
            }
            
            // Fit bounds to show all markers
            if !locs.is_empty() {
                let locations_json = serde_json::to_string(&locs).unwrap_or_default();
                fitMapBounds(&map, &locations_json);
            }
        }
    });

    // Update center when it changes
    if let Some(center_signal) = center {
        create_effect(move |_| {
            let center_data = center_signal.get();
            if let Some(map) = map_instance.get() {
                updateMapCenter(&map, center_data.latitude, center_data.longitude, center_data.zoom);
            }
        });
    }

    view! {
        <div
            node_ref=map_ref
            class=format!("mapbox-container {}", class)
            style="width: 100%; height: 100%; min-height: 400px; border-radius: var(--radius-xl); overflow: hidden; position: relative;"
        >
            <div class="map-loading" style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); z-index: 10;">
                <div class="loading-dots">
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            </div>
        </div>
    }
}

// Inject Mapbox initialization script
#[component]
pub fn MapboxScripts() -> impl IntoView {
    view! {
        <script src="https://api.mapbox.com/mapbox-gl-js/v3.0.1/mapbox-gl.js"></script>
        <link href="https://api.mapbox.com/mapbox-gl-js/v3.0.1/mapbox-gl.css" rel="stylesheet" />
        
        <script type="text/javascript">
            {r#"
            window.initializeMapbox = function(containerId, token, centerLat, centerLng, zoom) {
                mapboxgl.accessToken = token;
                
                const map = new mapboxgl.Map({
                    container: containerId,
                    style: 'mapbox://styles/mapbox/light-v11',
                    center: [centerLng, centerLat],
                    zoom: zoom,
                    attributionControl: false
                });

                // Add navigation controls
                map.addControl(new mapboxgl.NavigationControl(), 'top-right');
                
                // Store markers
                map.markers = new Map();

                // Hide loading spinner when map loads
                map.on('load', function() {
                    const container = document.getElementById(containerId);
                    const loading = container.querySelector('.map-loading');
                    if (loading) loading.style.display = 'none';
                });

                return map;
            };

            window.addMapboxMarker = function(map, id, lat, lng, title, description, isUrgent) {
                // Create custom marker element
                const el = document.createElement('div');
                el.className = 'map-marker' + (isUrgent ? ' urgent' : '');
                el.innerHTML = isUrgent ? '!' : '📍';
                
                // Create popup
                const popup = new mapboxgl.Popup({ 
                    offset: 25,
                    className: 'custom-popup'
                }).setHTML(`
                    <div class="job-card" style="padding: 1rem; min-width: 200px;">
                        <h3 class="apple-heading-3" style="margin-bottom: 0.5rem; font-size: 1.125rem;">${title}</h3>
                        ${description ? `<p class="apple-body" style="margin: 0; color: #666;">${description}</p>` : ''}
                        ${isUrgent ? '<span class="badge-urgent">Urgent</span>' : ''}
                    </div>
                `);

                // Create marker
                const marker = new mapboxgl.Marker(el)
                    .setLngLat([lng, lat])
                    .setPopup(popup)
                    .addTo(map);

                // Store marker reference
                map.markers.set(id, marker);

                // Add click handler
                el.addEventListener('click', () => {
                    if (window.onMapboxMarkerClick) {
                        window.onMapboxMarkerClick(id);
                    }
                });
            };

            window.clearMapboxMarkers = function(map) {
                if (map.markers) {
                    map.markers.forEach(marker => marker.remove());
                    map.markers.clear();
                }
            };

            window.updateMapCenter = function(map, lat, lng, zoom) {
                map.flyTo({
                    center: [lng, lat],
                    zoom: zoom,
                    duration: 1000
                });
            };

            window.fitMapBounds = function(map, locationsJson) {
                try {
                    const locations = JSON.parse(locationsJson);
                    if (locations.length === 0) return;

                    const bounds = new mapboxgl.LngLatBounds();
                    locations.forEach(loc => {
                        bounds.extend([loc.longitude, loc.latitude]);
                    });

                    map.fitBounds(bounds, {
                        padding: 50,
                        maxZoom: 12,
                        duration: 1000
                    });
                } catch (e) {
                    console.error('Error fitting bounds:', e);
                }
            };
            "#}
        </script>
        
        <style>
            {r#"
            .mapbox-container {
                background: rgba(255, 255, 255, 0.9);
                box-shadow: var(--shadow-lg);
                border: 1px solid rgba(0, 0, 0, 0.05);
            }

            .mapboxgl-popup-content {
                padding: 0 !important;
                background: transparent !important;
                box-shadow: none !important;
                border: none !important;
            }

            .mapboxgl-popup-tip {
                display: none !important;
            }

            .custom-popup .job-card {
                background: rgba(255, 255, 255, 0.95);
                backdrop-filter: blur(20px);
                border: 1px solid rgba(23, 221, 184, 0.2);
                border-radius: var(--radius-lg);
                box-shadow: var(--shadow-xl);
            }

            .mapboxgl-ctrl-group {
                background: rgba(255, 255, 255, 0.9);
                backdrop-filter: blur(10px);
                border: 1px solid rgba(0, 0, 0, 0.1);
                border-radius: var(--radius-md);
                box-shadow: var(--shadow-md);
            }

            .mapboxgl-ctrl-group button {
                border-color: rgba(0, 0, 0, 0.1);
            }

            .mapboxgl-ctrl-group button:hover {
                background-color: rgba(23, 221, 184, 0.1);
            }

            /* Map loading animation */
            .map-loading {
                background: rgba(255, 255, 255, 0.9);
                padding: 1rem;
                border-radius: var(--radius-md);
                box-shadow: var(--shadow-md);
            }
            "#}
        </style>
    }
}

// Helper function to get user's current location
pub async fn get_user_location() -> Result<(f64, f64), String> {
    // For now, return Sydney coordinates as default
    // Full geolocation implementation would require additional setup
    Ok((-33.8688, 151.2093))
}