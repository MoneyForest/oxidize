use oxidize_domain::{StaffId, StaffRole, TenantId};

#[derive(Debug)]
pub struct CreateStaffInput {
    pub tenant_id: TenantId,
    pub role: StaffRole,
    pub auth_uid: String,
    pub display_name: String,
    pub image_path: String,
    pub email: String,
}

#[derive(Debug)]
pub struct UpdateStaffInput {
    pub id: StaffId,
    pub role: Option<StaffRole>,
    pub display_name: Option<String>,
    pub image_path: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug)]
pub struct GetStaffInput {
    pub id: Option<StaffId>,
    pub auth_uid: Option<String>,
    pub with_tenant: bool,
}

#[derive(Debug, Default)]
pub struct ListStaffInput {
    pub tenant_id: Option<TenantId>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug)]
pub struct DeleteStaffInput {
    pub id: StaffId,
}
