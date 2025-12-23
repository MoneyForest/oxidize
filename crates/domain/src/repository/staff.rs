use async_trait::async_trait;

use crate::error::Result;
use crate::model::{Staff, StaffId, TenantId};

#[derive(Debug, Default)]
pub struct GetStaffQuery {
    pub id: Option<StaffId>,
    pub auth_uid: Option<String>,
    pub with_tenant: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ListStaffQuery {
    pub tenant_id: Option<TenantId>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[async_trait]
pub trait StaffRepository: Send + Sync {
    async fn get(&self, query: GetStaffQuery) -> Result<Option<Staff>>;
    async fn list(&self, query: ListStaffQuery) -> Result<Vec<Staff>>;
    async fn count(&self, query: ListStaffQuery) -> Result<u64>;
    async fn create(&self, staff: &Staff) -> Result<()>;
    async fn update(&self, staff: &Staff) -> Result<()>;
    async fn delete(&self, id: &StaffId) -> Result<()>;
}
