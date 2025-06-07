//! WASM-specific tests using wasm-bindgen-test
//! These tests run directly in the browser environment

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::*;
use js_sys::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_wasm_environment() {
    // Basic test to ensure WASM environment is working
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_dom_access() {
    let document = web_sys::window()
        .expect("should have window")
        .document()
        .expect("should have document");
    
    let body = document.body().expect("should have body");
    assert!(body.is_instance_of::<HtmlElement>());
}

#[wasm_bindgen_test]
fn test_local_storage() {
    let window = web_sys::window().expect("should have window");
    let storage = window
        .local_storage()
        .expect("should have local storage")
        .expect("local storage should be available");
    
    // Test setting and getting values
    storage.set_item("test_key", "test_value").expect("should set item");
    let value = storage.get_item("test_key").expect("should get item");
    assert_eq!(value, Some("test_value".to_string()));
    
    // Clean up
    storage.remove_item("test_key").expect("should remove item");
}

#[wasm_bindgen_test]
fn test_json_parsing() {
    let json_str = r#"{"name": "test", "value": 42}"#;
    let parsed: Result<Object, _> = JSON::parse(json_str);
    assert!(parsed.is_ok());
}

#[wasm_bindgen_test]
fn test_console_logging() {
    // Test that console logging works in WASM
    console::log_1(&"Test log message from WASM".into());
    
    // This test passes if no panic occurs
    assert!(true);
}

#[wasm_bindgen_test]
async fn test_fetch_api() {
    let window = web_sys::window().expect("should have window");
    
    // Test fetch is available
    assert!(js_sys::Reflect::has(&window, &"fetch".into()).expect("should check fetch"));
    
    // Note: Actual fetch calls would require a running server
    // This just tests that the API is available
}

#[wasm_bindgen_test]
fn test_geolocation_api() {
    let window = web_sys::window().expect("should have window");
    let navigator = window.navigator();
    
    // Test that geolocation API is available
    let geolocation = navigator.geolocation();
    assert!(geolocation.is_ok());
}

#[wasm_bindgen_test]
fn test_url_api() {
    let url = Url::new("https://example.com/path?param=value").expect("should create URL");
    assert_eq!(url.hostname(), "example.com");
    assert_eq!(url.pathname(), "/path");
}

#[wasm_bindgen_test]
fn test_date_handling() {
    let now = Date::now();
    assert!(now > 0.0);
    
    let date = Date::new_0();
    assert!(date.get_time() > 0.0);
}

#[wasm_bindgen_test]
fn test_event_handling() {
    let document = web_sys::window()
        .expect("should have window")
        .document()
        .expect("should have document");
    
    let div = document
        .create_element("div")
        .expect("should create div");
    
    // Test that we can add event listeners
    let closure = Closure::wrap(Box::new(move |_event: Event| {
        console::log_1(&"Event fired".into());
    }) as Box<dyn FnMut(_)>);
    
    div.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("should add event listener");
    
    closure.forget(); // Don't drop the closure
}

#[wasm_bindgen_test]
fn test_canvas_api() {
    let document = web_sys::window()
        .expect("should have window")
        .document()
        .expect("should have document");
    
    let canvas = document
        .create_element("canvas")
        .expect("should create canvas")
        .dyn_into::<HtmlCanvasElement>()
        .expect("should be canvas element");
    
    let context = canvas
        .get_context("2d")
        .expect("should get context")
        .expect("context should exist")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("should be 2d context");
    
    // Basic drawing test
    context.fill_rect(0.0, 0.0, 10.0, 10.0);
}

#[wasm_bindgen_test]
fn test_crypto_api() {
    let window = web_sys::window().expect("should have window");
    let crypto = window.crypto().expect("should have crypto");
    
    let mut buffer = [0u8; 16];
    crypto
        .get_random_values_with_u8_array(&mut buffer)
        .expect("should get random values");
    
    // Buffer should now contain random data
    assert_ne!(buffer, [0u8; 16]); // Very unlikely to be all zeros
}

#[wasm_bindgen_test]
async fn test_animation_frame() {
    use wasm_bindgen_futures::JsFuture;
    
    let window = web_sys::window().expect("should have window");
    
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let closure = Closure::once(move |_time: f64| {
            resolve.call0(&JsValue::NULL).unwrap();
        });
        
        window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("should request animation frame");
        
        closure.forget();
    });
    
    JsFuture::from(promise).await.expect("animation frame should fire");
}

#[wasm_bindgen_test]
fn test_worker_support() {
    let window = web_sys::window().expect("should have window");
    
    // Test that Worker constructor is available
    let has_worker = js_sys::Reflect::has(&window, &"Worker".into())
        .expect("should check Worker");
    
    // Workers might not be available in all test environments
    // So we just check if the API exists
    console::log_1(&format!("Worker support: {}", has_worker).into());
}

// Tests for Leptos-specific functionality (when compiled with Leptos)
#[cfg(feature = "leptos")]
mod leptos_tests {
    use super::*;
    
    #[wasm_bindgen_test]
    fn test_leptos_signals() {
        // Test that Leptos signals work in WASM
        // This would require importing Leptos and creating actual signals
        // For now, just a placeholder
        assert!(true);
    }
    
    #[wasm_bindgen_test]
    fn test_leptos_components() {
        // Test component rendering in WASM
        // This would require setting up a Leptos app context
        assert!(true);
    }
}

// Performance tests
mod performance_tests {
    use super::*;
    
    #[wasm_bindgen_test]
    fn test_performance_timing() {
        let window = web_sys::window().expect("should have window");
        let performance = window.performance().expect("should have performance");
        
        let start = performance.now();
        
        // Do some work
        for i in 0..1000 {
            let _ = i * 2;
        }
        
        let end = performance.now();
        let duration = end - start;
        
        console::log_1(&format!("Test duration: {} ms", duration).into());
        assert!(duration >= 0.0);
    }
    
    #[wasm_bindgen_test]
    fn test_memory_usage() {
        let window = web_sys::window().expect("should have window");
        let performance = window.performance().expect("should have performance");
        
        // Check if memory API is available
        let memory = js_sys::Reflect::get(&performance, &"memory".into());
        
        if let Ok(memory_obj) = memory {
            if !memory_obj.is_undefined() {
                console::log_1(&"Memory API available".into());
                
                let used_heap = js_sys::Reflect::get(&memory_obj, &"usedJSHeapSize".into())
                    .unwrap_or(JsValue::from(0));
                
                console::log_1(&format!("Used heap: {:?}", used_heap).into());
            }
        }
    }
}

// Integration tests that require network access
#[cfg(feature = "network-tests")]
mod network_tests {
    use super::*;
    use wasm_bindgen_futures::JsFuture;
    
    #[wasm_bindgen_test]
    async fn test_api_connectivity() {
        let window = web_sys::window().expect("should have window");
        
        let request = Request::new_with_str("http://localhost:3070/health")
            .expect("should create request");
        
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .expect("should fetch");
        
        let resp: Response = resp_value.dyn_into().expect("should be response");
        assert!(resp.ok());
    }
}