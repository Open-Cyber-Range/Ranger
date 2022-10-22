// @generated automatically by Diesel CLI.

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

diesel::joinable!(deployments -> exercises (exercise_id));

diesel::allow_tables_to_appear_in_same_query!(
    deployments,
    exercises,
);
