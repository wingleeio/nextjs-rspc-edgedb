use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    Router,
};

use core::context::{self, Context};
use rspc::integrations::httpz::Request;
use std::{
    error::Error,
    net::{Ipv6Addr, SocketAddr},
    sync::Mutex,
};
use tower_http::cors::{Any, CorsLayer};

mod core;
mod middleware;
mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let conn = edgedb_tokio::create_client().await?;
    let val: i64 = conn.query_required_single("select 1 + 1", &()).await?;

    println!("1 + 1 = {}", val);

    let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 4000));

    let router = router::get();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let mut ctx = Context::new();

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
        .with_max_level(tracing::Level::DEBUG)
        .init();

    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
