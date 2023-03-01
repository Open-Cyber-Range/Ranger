CREATE TABLE condition_messages (
    id BINARY(16) NOT NULL,
    deployment_id BINARY(16) NOT NULL,
    condition_id BINARY(16) NOT NULL,
    value DECIMAL(18, 17) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NOT NULL DEFAULT '1970-01-01 00:00:01',
    PRIMARY KEY (id),
    FOREIGN KEY (deployment_id) REFERENCES deployments(id)
)