use async_trait::async_trait;

use crate::error::Result;
use crate::model::{Tenant, TenantId};

#[derive(Debug, Default)]
pub struct GetTenantQuery {
    pub id: Option<TenantId>,
}

#[derive(Debug, Default, Clone)]
pub struct ListTenantQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[async_trait]
pub trait TenantRepository: Send + Sync {
    async fn get(&self, query: GetTenantQuery) -> Result<Option<Tenant>>;
    async fn list(&self, query: ListTenantQuery) -> Result<Vec<Tenant>>;
    async fn count(&self, query: ListTenantQuery) -> Result<u64>;
    async fn create(&self, tenant: &Tenant) -> Result<()>;
    async fn update(&self, tenant: &Tenant) -> Result<()>;
    async fn delete(&self, id: &TenantId) -> Result<()>;
}
