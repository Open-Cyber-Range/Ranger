// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Binary,
        template_id -> Binary,
        username -> Tinytext,
        password -> Nullable<Tinytext>,
        private_key -> Nullable<Text>,
        exercise_id -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    condition_messages (id) {
        id -> Binary,
        exercise_id -> Binary,
        deployment_id -> Binary,
        virtual_machine_id -> Binary,
        condition_name -> Tinytext,
        condition_id -> Binary,
        value -> Decimal,
        created_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    deployment_elements (id) {
        id -> Binary,
        deployment_id -> Binary,
        scenario_reference -> Tinytext,
        handler_reference -> Nullable<Tinytext>,
        deployer_type -> Tinytext,
        status -> Tinytext,
        executor_log -> Nullable<Mediumtext>,
        event_id -> Nullable<Binary>,
        parent_node_id -> Nullable<Binary>,
        created_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    deployments (id) {
        id -> Binary,
        name -> Tinytext,
        group_name -> Nullable<Tinytext>,
        deployment_group -> Nullable<Tinytext>,
        sdl_schema -> Longtext,
        exercise_id -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        id -> Binary,
        name -> Tinytext,
        start -> Timestamp,
        end -> Timestamp,
        is_scheduled -> Bool,
        has_triggered -> Bool,
        deployment_id -> Binary,
        parent_node_id -> Binary,
        triggered_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    exercises (id) {
        id -> Binary,
        name -> Tinytext,
        group_name -> Nullable<Tinytext>,
        sdl_schema -> Nullable<Longtext>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    participants (id) {
        id -> Binary,
        deployment_id -> Binary,
        user_id -> Text,
        selector -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::joinable!(accounts -> exercises (exercise_id));
diesel::joinable!(condition_messages -> deployments (deployment_id));
diesel::joinable!(deployment_elements -> deployments (deployment_id));
diesel::joinable!(deployment_elements -> events (event_id));
diesel::joinable!(deployments -> exercises (exercise_id));
diesel::joinable!(participants -> deployments (deployment_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    condition_messages,
    deployment_elements,
    deployments,
    events,
    exercises,
    participants,
);
