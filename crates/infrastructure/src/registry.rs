use std::sync::Arc;

use oxidize_usecase::{StaffInteractor, TenantInteractor};

use crate::database::{create_pool, StaffRepositoryImpl, TenantRepositoryImpl};

pub struct Registry {
    pub tenant_interactor: TenantInteractor<TenantRepositoryImpl>,
    pub staff_interactor: StaffInteractor<StaffRepositoryImpl>,
}

impl Registry {
    pub async fn new(database_url: &str) -> anyhow::Result<Arc<Self>> {
        let pool = create_pool(database_url).await?;

        let tenant_repo = Arc::new(TenantRepositoryImpl::new(pool.clone()));
        let staff_repo = Arc::new(StaffRepositoryImpl::new(pool));

        Ok(Arc::new(Self {
            tenant_interactor: TenantInteractor::new(tenant_repo),
            staff_interactor: StaffInteractor::new(staff_repo),
        }))
    }
}
