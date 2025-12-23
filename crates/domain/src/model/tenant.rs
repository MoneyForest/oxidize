use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::TenantTagType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantId(String);

impl TenantId {
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

impl Default for TenantId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantTagId(String);

impl TenantTagId {
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

impl Default for TenantTagId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TenantTag {
    pub id: TenantTagId,
    pub tag_type: TenantTagType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TenantTag {
    pub fn new(tag_type: TenantTagType, now: DateTime<Utc>) -> Self {
        Self {
            id: TenantTagId::new(),
            tag_type,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: TenantId,
    pub name: String,
    pub tags: Vec<TenantTag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tenant {
    pub fn new(name: String, now: DateTime<Utc>) -> Self {
        Self {
            id: TenantId::new(),
            name,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, name: Option<String>, now: DateTime<Utc>) {
        if let Some(n) = name {
            self.name = n;
        }
        self.updated_at = now;
    }

    pub fn add_tag(&mut self, tag: TenantTag) {
        self.tags.push(tag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tenant() {
        let now = Utc::now();
        let tenant = Tenant::new("Test Tenant".to_string(), now);

        assert_eq!(tenant.name, "Test Tenant");
        assert!(tenant.tags.is_empty());
    }

    #[test]
    fn test_update_tenant() {
        let now = Utc::now();
        let mut tenant = Tenant::new("Original".to_string(), now);

        let later = Utc::now();
        tenant.update(Some("Updated".to_string()), later);

        assert_eq!(tenant.name, "Updated");
        assert!(tenant.updated_at >= tenant.created_at);
    }
}
