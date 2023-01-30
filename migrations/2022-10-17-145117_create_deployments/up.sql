CREATE TABLE deployments (
    id BINARY(16) NOT NULL,
    name TINYTEXT NOT NULL,
    deployment_group TINYTEXT,
    sdl_schema LONGTEXT NOT NULL,
    exercise_id BINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL DEFAULT NULL,
    is_deployed BOOLEAN GENERATED ALWAYS AS (IF(deleted_at IS NULL, 1, NULL)),
    PRIMARY KEY (id),
    FOREIGN KEY (exercise_id) REFERENCES exercises(id),
    UNIQUE (name, exercise_id, is_deployed)
);