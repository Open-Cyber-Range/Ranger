CREATE TABLE events (
    id BINARY(16) NOT NULL,
    name TINYTEXT NOT NULL,
    start TIMESTAMP NOT NULL DEFAULT '1970-01-01 00:00:01',
    end TIMESTAMP NOT NULL DEFAULT '1970-01-01 00:00:01',
    deployment_id BINARY(16) NOT NULL,
    parent_node_id BINARY(16) NOT NULL,
    is_scheduled BOOLEAN NOT NULL DEFAULT FALSE,
    has_triggered BOOLEAN NOT NULL DEFAULT FALSE,
    triggered_at TIMESTAMP NOT NULL DEFAULT '1970-01-01 00:00:01',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NOT NULL DEFAULT '1970-01-01 00:00:01',
    PRIMARY KEY (id)
);

ALTER TABLE deployment_elements
    ADD event_id BINARY(16) DEFAULT NULL AFTER executor_log,
    ADD FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE;