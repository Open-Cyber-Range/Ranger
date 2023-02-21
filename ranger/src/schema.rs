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
        deployment_id -> Binary,
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
        created_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    deployments (id) {
        id -> Binary,
        name -> Tinytext,
        deployment_group -> Nullable<Tinytext>,
        sdl_schema -> Longtext,
        exercise_id -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    exercises (id) {
        id -> Binary,
        name -> Tinytext,
        sdl_schema -> Nullable<Longtext>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    packages (id) {
        id -> Binary,
        name -> Tinytext,
        version -> Tinytext,
        license -> Text,
        readme -> Mediumtext,
        readme_html -> Longtext,
        checksum -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    scores (id) {
        id -> Binary,
        exercise_id -> Binary,
        deployment_id -> Binary,
        tlo_name -> Tinytext,
        metric_name -> Tinytext,
        value -> Decimal,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(accounts -> exercises (exercise_id));
diesel::joinable!(condition_messages -> deployments (deployment_id));
diesel::joinable!(deployment_elements -> deployments (deployment_id));
diesel::joinable!(deployments -> exercises (exercise_id));
diesel::joinable!(scores -> deployments (deployment_id));
diesel::joinable!(scores -> exercises (exercise_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    condition_messages,
    deployment_elements,
    deployments,
    exercises,
    packages,
    scores,
);
