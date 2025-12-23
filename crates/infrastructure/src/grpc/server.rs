use std::sync::Arc;

use tonic::transport::Server;
use tower_http::trace::TraceLayer;

use oxidize_usecase::{StaffInteractor, TenantInteractor};

use super::staff_service::proto::staff_service_server::StaffServiceServer;
use super::staff_service::StaffServiceImpl;
use super::tenant_service::proto::tenant_service_server::TenantServiceServer;
use super::tenant_service::TenantServiceImpl;
use crate::database::{create_pool, StaffRepositoryImpl, TenantRepositoryImpl};

pub async fn run_grpc_server(port: u16, database_url: &str) -> anyhow::Result<()> {
    let pool = create_pool(database_url).await?;

    let staff_repo = Arc::new(StaffRepositoryImpl::new(pool.clone()));
    let tenant_repo = Arc::new(TenantRepositoryImpl::new(pool));

    let staff_interactor = StaffInteractor::new(staff_repo);
    let tenant_interactor = TenantInteractor::new(tenant_repo);

    let staff_service = StaffServiceImpl::new(staff_interactor);
    let tenant_service = TenantServiceImpl::new(tenant_interactor);

    let addr = format!("0.0.0.0:{}", port).parse()?;
    tracing::info!("Starting gRPC server on {}", addr);

    let trace_layer = TraceLayer::new_for_grpc();

    Server::builder()
        .layer(trace_layer)
        .add_service(StaffServiceServer::new(staff_service))
        .add_service(TenantServiceServer::new(tenant_service))
        .serve(addr)
        .await?;

    Ok(())
}
