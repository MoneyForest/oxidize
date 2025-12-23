use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{StaffRole, Tenant, TenantId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaffId(String);

impl StaffId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for StaffId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Staff {
    pub id: StaffId,
    pub tenant_id: TenantId,
    pub role: StaffRole,
    pub auth_uid: String,
    pub display_name: String,
    pub image_path: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Computed field (not stored in DB)
    pub image_url: Option<String>,
    // Readonly reference (loaded separately)
    pub tenant: Option<Tenant>,
}

impl Staff {
    pub fn new(
        tenant_id: TenantId,
        role: StaffRole,
        auth_uid: String,
        display_name: String,
        image_path: String,
        email: String,
        now: DateTime<Utc>,
    ) -> Self {
        Self {
            id: StaffId::new(),
            tenant_id,
            role,
            auth_uid,
            display_name,
            image_path,
            email,
            created_at: now,
            updated_at: now,
            image_url: None,
            tenant: None,
        }
    }

    pub fn set_image_url(&mut self, url: String) {
        self.image_url = Some(url);
    }

    pub fn set_tenant(&mut self, tenant: Tenant) {
        self.tenant = Some(tenant);
    }

    pub fn is_admin(&self) -> bool {
        self.role.is_admin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_staff() {
        let now = Utc::now();
        let staff = Staff::new(
            TenantId::new(),
            StaffRole::Normal,
            "auth123".to_string(),
            "John Doe".to_string(),
            "/images/john.png".to_string(),
            "john@example.com".to_string(),
            now,
        );

        assert_eq!(staff.display_name, "John Doe");
        assert_eq!(staff.role, StaffRole::Normal);
        assert!(!staff.is_admin());
    }

    #[test]
    fn test_admin_staff() {
        let now = Utc::now();
        let staff = Staff::new(
            TenantId::new(),
            StaffRole::Admin,
            "auth456".to_string(),
            "Admin User".to_string(),
            "".to_string(),
            "admin@example.com".to_string(),
            now,
        );

        assert!(staff.is_admin());
    }
}
