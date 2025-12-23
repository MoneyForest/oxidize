use std::sync::Arc;

use axum::{extract::Request, routing::get, Router};
use tower_http::trace::{DefaultOnResponse, MakeSpan, TraceLayer};
use tracing::{Level, Span};

use super::handlers;
use crate::registry::Registry;

#[derive(Clone)]
struct OtelMakeSpan;

impl MakeSpan<axum::body::Body> for OtelMakeSpan {
    fn make_span(&mut self, request: &Request<axum::body::Body>) -> Span {
        tracing::info_span!(
            "http_request",
            http.method = %request.method(),
            http.uri = %request.uri(),
            http.version = ?request.version(),
            otel.kind = "server",
            otel.status_code = tracing::field::Empty,
        )
    }
}

pub async fn run_http_server(port: u16, registry: Arc<Registry>) -> anyhow::Result<()> {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(OtelMakeSpan)
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/api/v1/tenants", get(handlers::list_tenants))
        .route("/api/v1/staffs", get(handlers::list_staffs))
        .layer(trace_layer)
        .with_state(registry);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
