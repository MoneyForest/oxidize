use std::sync::Arc;

use chrono::Utc;
use oxidize_domain::{errors, GetTenantQuery, ListTenantQuery, Result, Tenant, TenantRepository};

use crate::input::{
    CreateTenantInput, DeleteTenantInput, GetTenantInput, ListTenantInput, UpdateTenantInput,
};
use crate::output::ListTenantOutput;

pub struct TenantInteractor<R: TenantRepository> {
    repository: Arc<R>,
}

impl<R: TenantRepository> TenantInteractor<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get(&self, input: GetTenantInput) -> Result<Option<Tenant>> {
        let query = GetTenantQuery { id: Some(input.id) };
        self.repository.get(query).await
    }

    pub async fn list(&self, input: ListTenantInput) -> Result<ListTenantOutput> {
        let query = ListTenantQuery {
            limit: input.limit,
            offset: input.offset,
        };
        let tenants = self.repository.list(query.clone()).await?;
        let total_count = self.repository.count(query).await?;
        Ok(ListTenantOutput {
            tenants,
            total_count,
        })
    }

    pub async fn create(&self, input: CreateTenantInput) -> Result<Tenant> {
        let now = Utc::now();
        let tenant = Tenant::new(input.name, now);
        self.repository.create(&tenant).await?;
        Ok(tenant)
    }

    pub async fn update(&self, input: UpdateTenantInput) -> Result<Tenant> {
        let query = GetTenantQuery {
            id: Some(input.id.clone()),
        };
        let mut tenant = self
            .repository
            .get(query)
            .await?
            .ok_or_else(errors::tenant_not_found)?;

        let now = Utc::now();
        tenant.update(input.name, now);

        self.repository.update(&tenant).await?;
        Ok(tenant)
    }

    pub async fn delete(&self, input: DeleteTenantInput) -> Result<()> {
        self.repository.delete(&input.id).await
    }
}
