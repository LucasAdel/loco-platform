#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use loco_app::*;
    use tracing::*;
    use tracing_subscriber;

    // Set up tracing
    tracing_subscriber::fmt::init();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    info!("🦀 Starting Loco Platform frontend server");
    info!("🌐 Frontend listening on http://{}", &addr);

    // Build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Run the server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("✅ Frontend server ready");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
fn shell(options: leptos::LeptosOptions) -> impl leptos::IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // This is not needed for the leptos CSR.
}