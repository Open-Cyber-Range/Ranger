// @generated automatically by Diesel CLI.

diesel::table! {
    deployment_elements (id) {
        id -> Binary,
        deployment_id -> Binary,
        handler_reference -> Tinytext,
        deployer_type -> Tinytext,
        teared_down -> Bool,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
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
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    exercises (id) {
        id -> Binary,
        name -> Tinytext,
        sdl_schema -> Nullable<Longtext>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(deployment_elements -> deployments (deployment_id));
diesel::joinable!(deployments -> exercises (exercise_id));

diesel::allow_tables_to_appear_in_same_query!(
    deployment_elements,
    deployments,
    exercises,
);
