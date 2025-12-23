-- Create staffs table
CREATE TABLE IF NOT EXISTS staffs (
    id VARCHAR(36) PRIMARY KEY,
    tenant_id VARCHAR(36) NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL DEFAULT 'unknown',
    auth_uid VARCHAR(255) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    image_path VARCHAR(500) NOT NULL DEFAULT '',
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_staffs_tenant_id ON staffs(tenant_id);
CREATE INDEX idx_staffs_auth_uid ON staffs(auth_uid);
CREATE INDEX idx_staffs_created_at ON staffs(created_at DESC);
