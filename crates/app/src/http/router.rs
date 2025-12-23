use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::config::Config;
use oxidize_infrastructure::{create_pool, StaffRepositoryImpl, TenantRepositoryImpl};
use oxidize_usecase::{StaffInteractor, TenantInteractor};

use super::handlers;

pub struct AppState {
    pub staff_interactor: StaffInteractor<StaffRepositoryImpl>,
    pub tenant_interactor: TenantInteractor<TenantRepositoryImpl>,
}

pub async fn run_server(port: u16) -> anyhow::Result<()> {
    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await?;

    let staff_repo = Arc::new(StaffRepositoryImpl::new(pool.clone()));
    let tenant_repo = Arc::new(TenantRepositoryImpl::new(pool));

    let state = Arc::new(AppState {
        staff_interactor: StaffInteractor::new(staff_repo),
        tenant_interactor: TenantInteractor::new(tenant_repo),
    });

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/api/v1/tenants", get(handlers::list_tenants))
        .route("/api/v1/staffs", get(handlers::list_staffs))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting HTTP server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
