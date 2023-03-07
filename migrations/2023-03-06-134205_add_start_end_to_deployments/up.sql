ALTER TABLE deployments
ADD start_time TIMESTAMP NOT NULL
AFTER sdl_schema;
ALTER TABLE deployments
ADD end_time TIMESTAMP NOT NULL
AFTER start_time;