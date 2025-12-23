use std::sync::Arc;

use chrono::Utc;
use oxidize_domain::{errors, GetStaffQuery, ListStaffQuery, Result, Staff, StaffRepository};

use crate::input::{
    CreateStaffInput, DeleteStaffInput, GetStaffInput, ListStaffInput, UpdateStaffInput,
};
use crate::output::ListStaffOutput;

pub struct StaffInteractor<R: StaffRepository> {
    repository: Arc<R>,
}

impl<R: StaffRepository> StaffInteractor<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get(&self, input: GetStaffInput) -> Result<Option<Staff>> {
        let query = GetStaffQuery {
            id: input.id,
            auth_uid: input.auth_uid,
            with_tenant: input.with_tenant,
        };
        self.repository.get(query).await
    }

    pub async fn list(&self, input: ListStaffInput) -> Result<ListStaffOutput> {
        let query = ListStaffQuery {
            tenant_id: input.tenant_id,
            limit: input.limit,
            offset: input.offset,
        };
        let staff = self.repository.list(query.clone()).await?;
        let total_count = self.repository.count(query).await?;
        Ok(ListStaffOutput { staff, total_count })
    }

    pub async fn create(&self, input: CreateStaffInput) -> Result<Staff> {
        let now = Utc::now();
        let staff = Staff::new(
            input.tenant_id,
            input.role,
            input.auth_uid,
            input.display_name,
            input.image_path,
            input.email,
            now,
        );
        self.repository.create(&staff).await?;
        Ok(staff)
    }

    pub async fn update(&self, input: UpdateStaffInput) -> Result<Staff> {
        let query = GetStaffQuery {
            id: Some(input.id.clone()),
            ..Default::default()
        };
        let mut staff = self
            .repository
            .get(query)
            .await?
            .ok_or_else(errors::staff_not_found)?;

        if let Some(role) = input.role {
            staff.role = role;
        }
        if let Some(display_name) = input.display_name {
            staff.display_name = display_name;
        }
        if let Some(image_path) = input.image_path {
            staff.image_path = image_path;
        }
        if let Some(email) = input.email {
            staff.email = email;
        }
        staff.updated_at = Utc::now();

        self.repository.update(&staff).await?;
        Ok(staff)
    }

    pub async fn delete(&self, input: DeleteStaffInput) -> Result<()> {
        self.repository.delete(&input.id).await
    }
}
