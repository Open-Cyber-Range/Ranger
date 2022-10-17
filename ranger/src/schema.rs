// @generated automatically by Diesel CLI.

diesel::table! {
    deployments (id) {
        id -> Binary,
        name -> Text,
        scenario_id -> Binary,
        exercise_id -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    exercises (id) {
        id -> Binary,
        name -> Text,
        scenario_id -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    scenarios (id) {
        id -> Binary,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::joinable!(deployments -> exercises (exercise_id));
diesel::joinable!(deployments -> scenarios (scenario_id));
diesel::joinable!(exercises -> scenarios (scenario_id));

diesel::allow_tables_to_appear_in_same_query!(
    deployments,
    exercises,
    scenarios,
);
