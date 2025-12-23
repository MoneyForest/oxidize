use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use oxidize_domain::{GetTenantQuery, ListTenantQuery, Result, Tenant, TenantId, TenantRepository};

#[derive(Debug, sqlx::FromRow)]
struct TenantRow {
    id: String,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<TenantRow> for Tenant {
    fn from(row: TenantRow) -> Self {
        Self {
            id: TenantId::from_string(row.id),
            name: row.name,
            tags: Vec::new(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

pub struct TenantRepositoryImpl {
    pool: PgPool,
}

impl TenantRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantRepository for TenantRepositoryImpl {
    async fn get(&self, query: GetTenantQuery) -> Result<Option<Tenant>> {
        let row: Option<TenantRow> = if let Some(id) = &query.id {
            sqlx::query_as("SELECT * FROM tenants WHERE id = $1")
                .bind(id.as_str())
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?
        } else {
            None
        };

        Ok(row.map(Tenant::from))
    }

    async fn list(&self, query: ListTenantQuery) -> Result<Vec<Tenant>> {
        let sql = format!(
            "SELECT * FROM tenants ORDER BY created_at DESC{}{}",
            query
                .limit
                .map(|l| format!(" LIMIT {}", l))
                .unwrap_or_default(),
            query
                .offset
                .map(|o| format!(" OFFSET {}", o))
                .unwrap_or_default()
        );

        let rows: Vec<TenantRow> = sqlx::query_as(&sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?;

        Ok(rows.into_iter().map(Tenant::from).collect())
    }

    async fn count(&self, _query: ListTenantQuery) -> Result<u64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?;

        Ok(count as u64)
    }

    async fn create(&self, tenant: &Tenant) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO tenants (id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(tenant.id.as_str())
        .bind(&tenant.name)
        .bind(tenant.created_at)
        .bind(tenant.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?;

        Ok(())
    }

    async fn update(&self, tenant: &Tenant) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE tenants
            SET name = $2, updated_at = $3
            WHERE id = $1
            "#,
        )
        .bind(tenant.id.as_str())
        .bind(&tenant.name)
        .bind(tenant.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &TenantId) -> Result<()> {
        sqlx::query("DELETE FROM tenants WHERE id = $1")
            .bind(id.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| oxidize_domain::DomainError::internal("DB_ERROR", e.to_string()))?;

        Ok(())
    }
}
