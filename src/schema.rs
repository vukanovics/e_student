// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Unsigned<Integer>,
        year -> Unsigned<Integer>,
        name -> Varchar,
        url -> Varchar,
        professor -> Unsigned<Integer>,
    }
}

diesel::table! {
    enrolments (id) {
        id -> Unsigned<Integer>,
        course -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
    }
}

diesel::table! {
    grade_assignments (id) {
        id -> Unsigned<Integer>,
        course -> Unsigned<Integer>,
        name -> Varchar,
    }
}

diesel::table! {
    point_assignments (id) {
        id -> Unsigned<Integer>,
        course -> Unsigned<Integer>,
        name -> Varchar,
        max_points -> Unsigned<Integer>,
    }
}

diesel::table! {
    sessions (session_key) {
        session_key -> Binary,
        user_id -> Unsigned<Integer>,
        created_on -> Datetime,
        last_refreshed -> Datetime,
        timeout_duration_seconds -> Unsigned<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Integer>,
        password -> Varchar,
        email -> Varchar,
        account_type -> Unsigned<Tinyint>,
        password_reset_required -> Bool,
        username -> Nullable<Varchar>,
        last_login_time -> Nullable<Datetime>,
    }
}

diesel::joinable!(courses -> users (professor));
diesel::joinable!(enrolments -> courses (course));
diesel::joinable!(enrolments -> users (student));
diesel::joinable!(grade_assignments -> courses (course));
diesel::joinable!(point_assignments -> courses (course));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    enrolments,
    grade_assignments,
    point_assignments,
    sessions,
    users,
);
