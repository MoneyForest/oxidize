use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;

use oxidize_usecase::{ListStaffInput, ListStaffOutput, ListTenantInput, ListTenantOutput};

use crate::registry::Registry;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

#[tracing::instrument]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[derive(Serialize)]
pub struct ListTenantsResponse {
    tenants: Vec<TenantResponse>,
    total_count: u64,
}

#[derive(Serialize)]
pub struct TenantResponse {
    id: String,
    name: String,
}

#[tracing::instrument(skip(state))]
pub async fn list_tenants(
    State(state): State<Arc<Registry>>,
) -> Result<Json<ListTenantsResponse>, StatusCode> {
    let input = ListTenantInput::default();

    let output: ListTenantOutput = state
        .tenant_interactor
        .list(input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ListTenantsResponse {
        tenants: output
            .tenants
            .into_iter()
            .map(|t| TenantResponse {
                id: t.id.as_str().to_string(),
                name: t.name,
            })
            .collect(),
        total_count: output.total_count,
    };

    Ok(Json(response))
}

#[derive(Serialize)]
pub struct ListStaffsResponse {
    staffs: Vec<StaffResponse>,
    total_count: u64,
}

#[derive(Serialize)]
pub struct StaffResponse {
    id: String,
    tenant_id: String,
    display_name: String,
    email: String,
}

#[tracing::instrument(skip(state))]
pub async fn list_staffs(
    State(state): State<Arc<Registry>>,
) -> Result<Json<ListStaffsResponse>, StatusCode> {
    let input = ListStaffInput::default();

    let output: ListStaffOutput = state
        .staff_interactor
        .list(input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ListStaffsResponse {
        staffs: output
            .staff
            .into_iter()
            .map(|s| StaffResponse {
                id: s.id.as_str().to_string(),
                tenant_id: s.tenant_id.as_str().to_string(),
                display_name: s.display_name,
                email: s.email,
            })
            .collect(),
        total_count: output.total_count,
    };

    Ok(Json(response))
}
