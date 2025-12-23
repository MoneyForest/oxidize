use oxidize_domain::Tenant;

#[derive(Debug)]
pub struct ListTenantOutput {
    pub tenants: Vec<Tenant>,
    pub total_count: u64,
}
