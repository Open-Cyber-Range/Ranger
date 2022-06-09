
#[cfg(test)]
mod tests {


    fn setup_test_server() -> Result<(String, Data<AppState>)> {
        let app_state = create_test_app_state();
    }

    pub fn create_test_app_state() -> Data<AppState> {
        let configuration = create_test_configuration();
        let node_client_address = create_test_node_client_address();
        let database_address = create_test_database_address();
        let deployer_address = create_test_deployer_address();
        let app_state = AppState {
            database_address,
            deployer_address,
        };
        Data::new(app_state)
    }
}