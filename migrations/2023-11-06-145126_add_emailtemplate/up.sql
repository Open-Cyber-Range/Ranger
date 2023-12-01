CREATE TABLE email_templates (
    id BINARY(16) NOT NULL,
    name TINYTEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
)