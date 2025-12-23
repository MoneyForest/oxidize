use std::sync::Arc;

use tonic::transport::Server;

use crate::config::Config;
use oxidize_infrastructure::{create_pool, StaffRepositoryImpl, TenantRepositoryImpl};
use oxidize_usecase::{StaffInteractor, TenantInteractor};

use super::staff_service::proto::staff_service_server::StaffServiceServer;
use super::staff_service::StaffServiceImpl;
use super::tenant_service::proto::tenant_service_server::TenantServiceServer;
use super::tenant_service::TenantServiceImpl;

pub async fn run_grpc_server(port: u16) -> anyhow::Result<()> {
    let config = Config::from_env();
    let pool = create_pool(&config.database_url).await?;

    let staff_repo = Arc::new(StaffRepositoryImpl::new(pool.clone()));
    let tenant_repo = Arc::new(TenantRepositoryImpl::new(pool));

    let staff_interactor = StaffInteractor::new(staff_repo);
    let tenant_interactor = TenantInteractor::new(tenant_repo);

    let staff_service = StaffServiceImpl::new(staff_interactor);
    let tenant_service = TenantServiceImpl::new(tenant_interactor);

    let addr = format!("0.0.0.0:{}", port).parse()?;
    tracing::info!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(StaffServiceServer::new(staff_service))
        .add_service(TenantServiceServer::new(tenant_service))
        .serve(addr)
        .await?;

    Ok(())
}
