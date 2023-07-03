// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id, created) {
        id -> Unsigned<Integer>,
        created -> Datetime,
        year -> Unsigned<Integer>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
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
        #[max_length = 255]
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
        #[max_length = 255]
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
        #[max_length = 2]
        short_name -> Char,
        #[max_length = 64]
        full_name -> Varchar,
        deleted -> Bool,
    }
}

diesel::table! {
    sessions (session_key) {
        #[max_length = 32]
        session_key -> Binary,
        user -> Unsigned<Integer>,
        created_on -> Datetime,
        last_refreshed -> Datetime,
        timeout_duration_seconds -> Unsigned<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Integer>,
        #[max_length = 60]
        password -> Varchar,
        #[max_length = 320]
        email -> Varchar,
        account_type -> Unsigned<Tinyint>,
        password_reset_required -> Bool,
        #[max_length = 32]
        first_name -> Nullable<Varchar>,
        #[max_length = 32]
        last_name -> Nullable<Varchar>,
        last_login_time -> Nullable<Datetime>,
        deleted -> Bool,
    }
}

diesel::table! {
    users_revisions (id, revision) {
        id -> Unsigned<Integer>,
        revision -> Unsigned<Integer>,
        created -> Nullable<Datetime>,
        #[max_length = 60]
        password -> Varchar,
        #[max_length = 320]
        email -> Varchar,
        account_type -> Unsigned<Tinyint>,
        password_reset_required -> Bool,
        #[max_length = 32]
        first_name -> Nullable<Varchar>,
        #[max_length = 32]
        last_name -> Nullable<Varchar>,
        last_login_time -> Nullable<Datetime>,
        deleted -> Bool,
    }
}

diesel::joinable!(courses -> users (professor));
diesel::joinable!(enrolments -> users (student));
diesel::joinable!(grade_assignments_progress -> users (student));
diesel::joinable!(indicies -> users (student));
diesel::joinable!(point_assignments_progress -> users (student));
diesel::joinable!(sessions -> users (user));
diesel::joinable!(users_revisions -> users (id));

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
    users_revisions,
);
