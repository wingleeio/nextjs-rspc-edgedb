use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};

use core::context::{self, Context};
use rspc::integrations::httpz::Request;
use std::{
    error::Error,
    net::{Ipv6Addr, SocketAddr},
    sync::{Arc, Mutex},
};
use tower_http::cors::CorsLayer;

mod core;
mod middleware;
mod router;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client = Arc::new(edgedb_tokio::create_client().await?);

    let auth = service::auth::Auth::new(client.clone());
    let users = service::users::Users::new(client.clone());

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 4000));

    let router = router::get();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin("http://localtest.me:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    let mut ctx = Context::new();

    context::add!(ctx, auth);
    context::add!(ctx, users);

    let app = Router::new()
        .nest(
            "/",
            router
                .endpoint(|req: Request| {
                    context::add!(ctx, Mutex::new(req));
                    ctx
                })
                .axum(),
        )
        .layer(cors);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
