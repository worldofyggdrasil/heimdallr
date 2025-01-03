CREATE TABLE IF NOT EXISTS oidc_clients (
    id SERIAL PRIMARY KEY,
    client_id VARCHAR(255) NOT NULL UNIQUE,
    client_secret VARCHAR(255) NOT NULL,
    redirect_uri VARCHAR(2048) NOT NULL,
    scope VARCHAR(255) NOT NULL,
    allowed_grant_types VARCHAR(255) NOT NULL,
    allowed_response_types VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
-- Create indexes for common lookups
CREATE INDEX idx_oidc_clients_client_id ON oidc_clients(client_id);
-- Add comment to table
COMMENT ON TABLE oidc_clients IS 'OIDC clients for authentication and authorization';