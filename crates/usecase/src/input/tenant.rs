use oxidize_domain::TenantId;

#[derive(Debug)]
pub struct CreateTenantInput {
    pub name: String,
}

#[derive(Debug)]
pub struct UpdateTenantInput {
    pub id: TenantId,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct GetTenantInput {
    pub id: TenantId,
}

#[derive(Debug, Default)]
pub struct ListTenantInput {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug)]
pub struct DeleteTenantInput {
    pub id: TenantId,
}
