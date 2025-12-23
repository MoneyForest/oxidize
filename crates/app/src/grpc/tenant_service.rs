use tonic::{Request, Response, Status};

use oxidize_domain::TenantId;
use oxidize_infrastructure::TenantRepositoryImpl;
use oxidize_usecase::{
    CreateTenantInput, DeleteTenantInput, GetTenantInput, ListTenantInput, TenantInteractor,
    UpdateTenantInput,
};

pub mod proto {
    tonic::include_proto!("tenant");
}

use proto::tenant_service_server::TenantService;
use proto::{
    CreateTenantRequest, CreateTenantResponse, DeleteTenantRequest, DeleteTenantResponse,
    GetTenantRequest, GetTenantResponse, ListTenantsRequest, ListTenantsResponse, Tenant,
    UpdateTenantRequest, UpdateTenantResponse,
};

pub struct TenantServiceImpl {
    interactor: TenantInteractor<TenantRepositoryImpl>,
}

impl TenantServiceImpl {
    pub fn new(interactor: TenantInteractor<TenantRepositoryImpl>) -> Self {
        Self { interactor }
    }
}

fn to_proto_tenant(t: oxidize_domain::Tenant) -> Tenant {
    Tenant {
        id: t.id.as_str().to_string(),
        name: t.name,
        created_at: t.created_at.to_rfc3339(),
        updated_at: t.updated_at.to_rfc3339(),
    }
}

#[tonic::async_trait]
impl TenantService for TenantServiceImpl {
    async fn get_tenant(
        &self,
        request: Request<GetTenantRequest>,
    ) -> Result<Response<GetTenantResponse>, Status> {
        let req = request.into_inner();
        let input = GetTenantInput {
            id: TenantId::from_string(req.id),
        };

        let tenant = self
            .interactor
            .get(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Tenant not found"))?;

        Ok(Response::new(GetTenantResponse {
            tenant: Some(to_proto_tenant(tenant)),
        }))
    }

    async fn list_tenants(
        &self,
        request: Request<ListTenantsRequest>,
    ) -> Result<Response<ListTenantsResponse>, Status> {
        let req = request.into_inner();
        let input = ListTenantInput {
            limit: req.limit,
            offset: req.offset,
        };

        let output = self
            .interactor
            .list(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListTenantsResponse {
            tenants: output.tenants.into_iter().map(to_proto_tenant).collect(),
            total_count: output.total_count,
        }))
    }

    async fn create_tenant(
        &self,
        request: Request<CreateTenantRequest>,
    ) -> Result<Response<CreateTenantResponse>, Status> {
        let req = request.into_inner();
        let input = CreateTenantInput { name: req.name };

        let tenant = self
            .interactor
            .create(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateTenantResponse {
            tenant: Some(to_proto_tenant(tenant)),
        }))
    }

    async fn update_tenant(
        &self,
        request: Request<UpdateTenantRequest>,
    ) -> Result<Response<UpdateTenantResponse>, Status> {
        let req = request.into_inner();
        let input = UpdateTenantInput {
            id: TenantId::from_string(req.id),
            name: req.name,
        };

        let tenant = self
            .interactor
            .update(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateTenantResponse {
            tenant: Some(to_proto_tenant(tenant)),
        }))
    }

    async fn delete_tenant(
        &self,
        request: Request<DeleteTenantRequest>,
    ) -> Result<Response<DeleteTenantResponse>, Status> {
        let req = request.into_inner();
        let input = DeleteTenantInput {
            id: TenantId::from_string(req.id),
        };

        self.interactor
            .delete(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteTenantResponse {}))
    }
}
