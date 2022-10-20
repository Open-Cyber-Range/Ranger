CREATE TABLE deployments (
    id BINARY(16) NOT NULL,
    name TEXT NOT NULL,
    deployment_group TEXT,
    scenario_id BINARY(16) NOT NULL,
    exercise_id BINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL DEFAULT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (scenario_id) REFERENCES scenarios(id),
    FOREIGN KEY (exercise_id) REFERENCES exercises(id)
);
