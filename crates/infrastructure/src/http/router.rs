use std::sync::Arc;

use axum::{extract::Request, routing::get, Router};
use tower_http::trace::{DefaultOnResponse, MakeSpan, TraceLayer};
use tracing::{Level, Span};

use oxidize_usecase::{StaffInteractor, TenantInteractor};

use super::handlers;
use crate::database::{create_pool, StaffRepositoryImpl, TenantRepositoryImpl};

pub struct AppState {
    pub staff_interactor: StaffInteractor<StaffRepositoryImpl>,
    pub tenant_interactor: TenantInteractor<TenantRepositoryImpl>,
}

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

pub async fn run_http_server(port: u16, database_url: &str) -> anyhow::Result<()> {
    let pool = create_pool(database_url).await?;

    let staff_repo = Arc::new(StaffRepositoryImpl::new(pool.clone()));
    let tenant_repo = Arc::new(TenantRepositoryImpl::new(pool));

    let state = Arc::new(AppState {
        staff_interactor: StaffInteractor::new(staff_repo),
        tenant_interactor: TenantInteractor::new(tenant_repo),
    });

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(OtelMakeSpan)
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/api/v1/tenants", get(handlers::list_tenants))
        .route("/api/v1/staffs", get(handlers::list_staffs))
        .layer(trace_layer)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
