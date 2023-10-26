// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 16]
        template_id -> Binary,
        username -> Tinytext,
        password -> Nullable<Tinytext>,
        private_key -> Nullable<Text>,
        #[max_length = 16]
        exercise_id -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    artifacts (id) {
        #[max_length = 16]
        id -> Binary,
        name -> Tinytext,
        content -> Mediumblob,
        #[max_length = 16]
        metric_id -> Binary,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    banners (exercise_id) {
        #[max_length = 16]
        exercise_id -> Binary,
        name -> Tinytext,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    condition_messages (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 16]
        exercise_id -> Binary,
        #[max_length = 16]
        deployment_id -> Binary,
        #[max_length = 16]
        virtual_machine_id -> Binary,
        condition_name -> Tinytext,
        #[max_length = 16]
        condition_id -> Binary,
        value -> Decimal,
        created_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    deployment_elements (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 16]
        deployment_id -> Binary,
        scenario_reference -> Tinytext,
        handler_reference -> Nullable<Tinytext>,
        deployer_type -> Tinytext,
        status -> Tinytext,
        executor_log -> Nullable<Mediumtext>,
        #[max_length = 16]
        event_id -> Nullable<Binary>,
        #[max_length = 16]
        parent_node_id -> Nullable<Binary>,
        created_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    deployments (id) {
        #[max_length = 16]
        id -> Binary,
        name -> Tinytext,
        group_name -> Nullable<Tinytext>,
        deployment_group -> Nullable<Tinytext>,
        sdl_schema -> Longtext,
        #[max_length = 16]
        exercise_id -> Binary,
        start -> Timestamp,
        end -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        #[max_length = 16]
        id -> Binary,
        name -> Tinytext,
        start -> Timestamp,
        end -> Timestamp,
        #[max_length = 16]
        deployment_id -> Binary,
        #[max_length = 16]
        parent_node_id -> Binary,
        description -> Nullable<Mediumtext>,
        has_triggered -> Bool,
        triggered_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    exercises (id) {
        #[max_length = 16]
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
    metrics (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 16]
        exercise_id -> Binary,
        #[max_length = 16]
        deployment_id -> Binary,
        entity_selector -> Text,
        name -> Text,
        description -> Nullable<Text>,
        role -> Tinytext,
        text_submission -> Nullable<Text>,
        score -> Nullable<Unsigned<Integer>>,
        max_score -> Unsigned<Integer>,
        has_artifact -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    participants (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 16]
        deployment_id -> Binary,
        user_id -> Text,
        selector -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::joinable!(accounts -> exercises (exercise_id));
diesel::joinable!(artifacts -> metrics (metric_id));
diesel::joinable!(banners -> exercises (exercise_id));
diesel::joinable!(condition_messages -> deployments (deployment_id));
diesel::joinable!(deployment_elements -> deployments (deployment_id));
diesel::joinable!(deployment_elements -> events (event_id));
diesel::joinable!(deployments -> exercises (exercise_id));
diesel::joinable!(metrics -> deployments (deployment_id));
diesel::joinable!(participants -> deployments (deployment_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    artifacts,
    banners,
    condition_messages,
    deployment_elements,
    deployments,
    events,
    exercises,
    metrics,
    participants,
);
