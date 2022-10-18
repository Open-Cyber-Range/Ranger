CREATE TABLE deployments (
    id BINARY(16) NOT NULL,
    name TEXT NOT NULL,
    scenario_id BINARY(16) NOT NULL,
    exercise_id BINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    PRIMARY KEY (id),
    FOREIGN KEY (scenario_id) REFERENCES scenarios(id),
    FOREIGN KEY (exercise_id) REFERENCES exercises(id)
);
