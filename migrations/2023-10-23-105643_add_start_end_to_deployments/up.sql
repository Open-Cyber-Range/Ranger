ALTER TABLE deployments
ADD COLUMN start TIMESTAMP NOT NULL DEFAULT '1970-01-01 00:00:01',
ADD COLUMN end TIMESTAMP NOT NULL DEFAULT '1970-01-01 00:00:01';