use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use oxidize_domain::{
    GetStaffQuery, ListStaffQuery, Result, Staff, StaffId, StaffRepository, TenantId,
};

#[derive(Debug, sqlx::FromRow)]
struct StaffRow {
    id: String,
    tenant_id: String,
    role: String,
    auth_uid: String,
    display_name: String,
    image_path: String,
    email: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<StaffRow> for Staff {
    fn from(row: StaffRow) -> Self {
        Self {
            id: StaffId::from_string(row.id),
            tenant_id: TenantId::from_string(row.tenant_id),
            role: row.role.parse().unwrap_or_default(),
            auth_uid: row.auth_uid,
            display_name: row.display_name,
            image_path: row.image_path,
            email: row.email,
            created_at: row.created_at,
            updated_at: row.updated_at,
            image_url: None,
            tenant: None,
        }
    }
}

pub struct StaffRepositoryImpl {
    pool: PgPool,
}

impl StaffRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StaffRepository for StaffRepositoryImpl {
    async fn get(&self, query: GetStaffQuery) -> Result<Option<Staff>> {
        let mut sql = String::from("SELECT * FROM staffs WHERE 1=1");

        if query.id.is_some() {
            sql.push_str(" AND id = $1");
        }
        if query.auth_uid.is_some() {
            sql.push_str(" AND auth_uid = $2");
        }

        let row: Option<StaffRow> = match (&query.id, &query.auth_uid) {
            (Some(id), Some(auth_uid)) => sqlx::query_as(&sql)
                .bind(id.as_str())
                .bind(auth_uid)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?,
            (Some(id), None) => {
                let sql = "SELECT * FROM staffs WHERE id = $1";
                sqlx::query_as(sql)
                    .bind(id.as_str())
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?
            }
            (None, Some(auth_uid)) => {
                let sql = "SELECT * FROM staffs WHERE auth_uid = $1";
                sqlx::query_as(sql)
                    .bind(auth_uid)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?
            }
            (None, None) => None,
        };

        Ok(row.map(Staff::from))
    }

    async fn list(&self, query: ListStaffQuery) -> Result<Vec<Staff>> {
        let mut sql = String::from("SELECT * FROM staffs WHERE 1=1");

        if query.tenant_id.is_some() {
            sql.push_str(" AND tenant_id = $1");
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = query.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = query.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let rows: Vec<StaffRow> = if let Some(tenant_id) = &query.tenant_id {
            sqlx::query_as(&sql)
                .bind(tenant_id.as_str())
                .fetch_all(&self.pool)
                .await
                .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?
        } else {
            let sql = format!(
                "SELECT * FROM staffs ORDER BY created_at DESC{}{}",
                query
                    .limit
                    .map(|l| format!(" LIMIT {}", l))
                    .unwrap_or_default(),
                query
                    .offset
                    .map(|o| format!(" OFFSET {}", o))
                    .unwrap_or_default()
            );
            sqlx::query_as(&sql)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?
        };

        Ok(rows.into_iter().map(Staff::from).collect())
    }

    async fn count(&self, query: ListStaffQuery) -> Result<u64> {
        let count: i64 = if let Some(tenant_id) = &query.tenant_id {
            sqlx::query_scalar("SELECT COUNT(*) FROM staffs WHERE tenant_id = $1")
                .bind(tenant_id.as_str())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?
        } else {
            sqlx::query_scalar("SELECT COUNT(*) FROM staffs")
                .fetch_one(&self.pool)
                .await
                .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?
        };

        Ok(count as u64)
    }

    async fn create(&self, staff: &Staff) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO staffs (id, tenant_id, role, auth_uid, display_name, image_path, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(staff.id.as_str())
        .bind(staff.tenant_id.as_str())
        .bind(staff.role.as_str())
        .bind(&staff.auth_uid)
        .bind(&staff.display_name)
        .bind(&staff.image_path)
        .bind(&staff.email)
        .bind(staff.created_at)
        .bind(staff.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?;

        Ok(())
    }

    async fn update(&self, staff: &Staff) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE staffs
            SET role = $2, display_name = $3, image_path = $4, email = $5, updated_at = $6
            WHERE id = $1
            "#,
        )
        .bind(staff.id.as_str())
        .bind(staff.role.as_str())
        .bind(&staff.display_name)
        .bind(&staff.image_path)
        .bind(&staff.email)
        .bind(staff.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &StaffId) -> Result<()> {
        sqlx::query("DELETE FROM staffs WHERE id = $1")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?;

        Ok(())
    }
}
