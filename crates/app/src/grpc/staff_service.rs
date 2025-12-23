use tonic::{Request, Response, Status};

use oxidize_domain::{StaffId, StaffRole, TenantId};
use oxidize_infrastructure::StaffRepositoryImpl;
use oxidize_usecase::{
    CreateStaffInput, DeleteStaffInput, GetStaffInput, ListStaffInput, StaffInteractor,
    UpdateStaffInput,
};

pub mod proto {
    tonic::include_proto!("staff");
}

use proto::staff_service_server::StaffService;
use proto::{
    CreateStaffRequest, CreateStaffResponse, DeleteStaffRequest, DeleteStaffResponse,
    GetStaffRequest, GetStaffResponse, ListStaffsRequest, ListStaffsResponse, Staff,
    UpdateStaffRequest, UpdateStaffResponse,
};

pub struct StaffServiceImpl {
    interactor: StaffInteractor<StaffRepositoryImpl>,
}

impl StaffServiceImpl {
    pub fn new(interactor: StaffInteractor<StaffRepositoryImpl>) -> Self {
        Self { interactor }
    }
}

fn to_proto_staff(s: oxidize_domain::Staff) -> Staff {
    Staff {
        id: s.id.as_str().to_string(),
        tenant_id: s.tenant_id.as_str().to_string(),
        role: s.role.to_string(),
        auth_uid: s.auth_uid,
        display_name: s.display_name,
        image_path: s.image_path,
        email: s.email,
        created_at: s.created_at.to_rfc3339(),
        updated_at: s.updated_at.to_rfc3339(),
    }
}

fn parse_role(role: &str) -> StaffRole {
    match role.to_lowercase().as_str() {
        "admin" => StaffRole::Admin,
        "normal" => StaffRole::Normal,
        _ => StaffRole::Unknown,
    }
}

#[tonic::async_trait]
impl StaffService for StaffServiceImpl {
    #[tracing::instrument(skip(self, request), fields(service = "staff"))]
    async fn get_staff(
        &self,
        request: Request<GetStaffRequest>,
    ) -> Result<Response<GetStaffResponse>, Status> {
        let req = request.into_inner();
        let input = GetStaffInput {
            id: req.id.map(StaffId::from_string),
            auth_uid: req.auth_uid,
            with_tenant: false,
        };

        let staff = self
            .interactor
            .get(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("Staff not found"))?;

        Ok(Response::new(GetStaffResponse {
            staff: Some(to_proto_staff(staff)),
        }))
    }

    #[tracing::instrument(skip(self, request), fields(service = "staff"))]
    async fn list_staffs(
        &self,
        request: Request<ListStaffsRequest>,
    ) -> Result<Response<ListStaffsResponse>, Status> {
        let req = request.into_inner();
        let input = ListStaffInput {
            tenant_id: req.tenant_id.map(TenantId::from_string),
            limit: req.limit,
            offset: req.offset,
        };

        let output = self
            .interactor
            .list(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListStaffsResponse {
            staffs: output.staff.into_iter().map(to_proto_staff).collect(),
            total_count: output.total_count,
        }))
    }

    #[tracing::instrument(skip(self, request), fields(service = "staff"))]
    async fn create_staff(
        &self,
        request: Request<CreateStaffRequest>,
    ) -> Result<Response<CreateStaffResponse>, Status> {
        let req = request.into_inner();
        let input = CreateStaffInput {
            tenant_id: TenantId::from_string(req.tenant_id),
            role: parse_role(&req.role),
            auth_uid: req.auth_uid,
            display_name: req.display_name,
            image_path: req.image_path,
            email: req.email,
        };

        let staff = self
            .interactor
            .create(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateStaffResponse {
            staff: Some(to_proto_staff(staff)),
        }))
    }

    #[tracing::instrument(skip(self, request), fields(service = "staff"))]
    async fn update_staff(
        &self,
        request: Request<UpdateStaffRequest>,
    ) -> Result<Response<UpdateStaffResponse>, Status> {
        let req = request.into_inner();
        let input = UpdateStaffInput {
            id: StaffId::from_string(req.id),
            role: req.role.map(|r| parse_role(&r)),
            display_name: req.display_name,
            image_path: req.image_path,
            email: req.email,
        };

        let staff = self
            .interactor
            .update(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateStaffResponse {
            staff: Some(to_proto_staff(staff)),
        }))
    }

    #[tracing::instrument(skip(self, request), fields(service = "staff"))]
    async fn delete_staff(
        &self,
        request: Request<DeleteStaffRequest>,
    ) -> Result<Response<DeleteStaffResponse>, Status> {
        let req = request.into_inner();
        let input = DeleteStaffInput {
            id: StaffId::from_string(req.id),
        };

        self.interactor
            .delete(input)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteStaffResponse {}))
    }
}
