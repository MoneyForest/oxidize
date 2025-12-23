use tonic::transport::Server;
use tower_http::trace::TraceLayer;

use oxidize_usecase::{StaffInteractor, TenantInteractor};

use super::staff_service::proto::staff_service_server::StaffServiceServer;
use super::staff_service::StaffServiceImpl;
use super::tenant_service::proto::tenant_service_server::TenantServiceServer;
use super::tenant_service::TenantServiceImpl;
use crate::database::{StaffRepositoryImpl, TenantRepositoryImpl};

pub async fn run_grpc_server(
    port: u16,
    tenant_interactor: TenantInteractor<TenantRepositoryImpl>,
    staff_interactor: StaffInteractor<StaffRepositoryImpl>,
) -> anyhow::Result<()> {
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
