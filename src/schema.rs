// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        year -> Unsigned<Integer>,
        name -> Varchar,
        url -> Varchar,
        professor -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    enrolments (course, student) {
        course -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
    }
}

diesel::table! {
    generations (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        year -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    grade_assignments (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        course -> Unsigned<Integer>,
        name -> Varchar,
        deleted -> Bool,
    }
}

diesel::table! {
    grade_assignments_progress (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        assignment -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
        grade -> Float,
    }
}

diesel::table! {
    indicies (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        program -> Unsigned<Integer>,
        generation -> Unsigned<Integer>,
        number -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    point_assignments (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        course -> Unsigned<Integer>,
        name -> Varchar,
        max_points -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    point_assignments_progress (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        assignment -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
        points -> Unsigned<Integer>,
    }
}

diesel::table! {
    programs (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        short_name -> Char,
        full_name -> Varchar,
        deleted -> Bool,
    }
}

diesel::table! {
    sessions (session_key) {
        session_key -> Binary,
        user -> Unsigned<Integer>,
        created_on -> Datetime,
        last_refreshed -> Datetime,
        timeout_duration_seconds -> Unsigned<Integer>,
    }
}

diesel::table! {
    users (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        password -> Varchar,
        email -> Varchar,
        account_type -> Unsigned<Tinyint>,
        password_reset_required -> Bool,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        last_login_time -> Nullable<Datetime>,
        deleted -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    enrolments,
    generations,
    grade_assignments,
    grade_assignments_progress,
    indicies,
    point_assignments,
    point_assignments_progress,
    programs,
    sessions,
    users,
);
