use std::sync::Arc;

use oxidize_infrastructure::{create_pool, AppState, StaffRepositoryImpl, TenantRepositoryImpl};
use oxidize_usecase::{StaffInteractor, TenantInteractor};

pub struct Registry {
    pub tenant_interactor: TenantInteractor<TenantRepositoryImpl>,
    pub staff_interactor: StaffInteractor<StaffRepositoryImpl>,
}

impl Registry {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = create_pool(database_url).await?;

        let tenant_repo = Arc::new(TenantRepositoryImpl::new(pool.clone()));
        let staff_repo = Arc::new(StaffRepositoryImpl::new(pool));

        Ok(Self {
            tenant_interactor: TenantInteractor::new(tenant_repo),
            staff_interactor: StaffInteractor::new(staff_repo),
        })
    }

    pub fn into_http_state(self) -> Arc<AppState> {
        Arc::new(AppState {
            tenant_interactor: self.tenant_interactor,
            staff_interactor: self.staff_interactor,
        })
    }
}
