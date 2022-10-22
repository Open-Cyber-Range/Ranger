CREATE TABLE deployment_elements (
  id BINARY(16) NOT NULL,
  deployment_id BINARY(16) NOT NULL,
  handler_reference TINYTEXT NOT NULL,
  deployer_type TINYTEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP NULL DEFAULT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (deployment_id) REFERENCES deployments(id)
);