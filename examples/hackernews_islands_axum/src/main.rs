#[cfg(feature = "ssr")]
mod ssr_imports {
    pub use axum::{routing::get, Router};
    pub use leptos::*;
    pub use leptos_axum::{generate_route_list, LeptosRoutes};
    pub use tower_http::{compression::CompressionLayer, services::ServeFile};
    pub use hackernews::fallback::file_and_error_handler;
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use hackernews::*;
    use ssr_imports::*;

    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|| view! {  <App/> }).await;

    // build our application with a route
    let app = Router::new()
        .route("/favicon.ico", get(file_and_error_handler))
        .leptos_routes(&leptos_options, routes, || view! {  <App/> } )
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
