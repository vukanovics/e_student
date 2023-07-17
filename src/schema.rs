// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Unsigned<Integer>,
        year -> Unsigned<Integer>,
        name -> Varchar,
        url -> Varchar,
        professor -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    courses_revisions (id, revision) {
        id -> Unsigned<Integer>,
        revision -> Unsigned<Integer>,
        created -> Nullable<Datetime>,
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
    generations (id) {
        id -> Unsigned<Integer>,
        year -> Unsigned<Integer>,
    }
}

diesel::table! {
    grade_assignments (id) {
        id -> Unsigned<Integer>,
        course -> Unsigned<Integer>,
        name -> Varchar,
        url -> Varchar,
        deleted -> Bool,
    }
}

diesel::table! {
    grade_assignments_progress (assignment, student) {
        assignment -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
        grade -> Float,
        deleted -> Bool,
    }
}

diesel::table! {
    grade_assignments_progress_revisions (assignment, student, revision) {
        assignment -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
        revision -> Unsigned<Integer>,
        created -> Nullable<Datetime>,
        grade -> Float,
        deleted -> Bool,
    }
}

diesel::table! {
    grade_assignments_revisions (id, revision) {
        id -> Unsigned<Integer>,
        revision -> Unsigned<Integer>,
        created -> Nullable<Datetime>,
        course -> Unsigned<Integer>,
        name -> Varchar,
        url -> Varchar,
        deleted -> Bool,
    }
}

diesel::table! {
    indicies (id) {
        id -> Unsigned<Integer>,
        program -> Unsigned<Integer>,
        generation -> Unsigned<Integer>,
        number -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
    }
}

diesel::table! {
    point_assignments (id) {
        id -> Unsigned<Integer>,
        course -> Unsigned<Integer>,
        name -> Varchar,
        url -> Varchar,
        max_points -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    point_assignments_progress (assignment, student) {
        assignment -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
        points -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    point_assignments_progress_revisions (assignment, student, revision) {
        assignment -> Unsigned<Integer>,
        student -> Unsigned<Integer>,
        revision -> Unsigned<Integer>,
        created -> Nullable<Datetime>,
        points -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    point_assignments_revisions (id, revision) {
        id -> Unsigned<Integer>,
        revision -> Unsigned<Integer>,
        created -> Nullable<Datetime>,
        course -> Unsigned<Integer>,
        name -> Varchar,
        url -> Varchar,
        max_points -> Unsigned<Integer>,
        deleted -> Bool,
    }
}

diesel::table! {
    programs (id) {
        id -> Unsigned<Integer>,
        short_name -> Char,
        full_name -> Varchar,
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
    users (id) {
        id -> Unsigned<Integer>,
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

diesel::table! {
    users_revisions (id, revision) {
        id -> Unsigned<Integer>,
        revision -> Unsigned<Integer>,
        created -> Nullable<Datetime>,
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

diesel::joinable!(courses -> users (professor));
diesel::joinable!(courses_revisions -> courses (id));
diesel::joinable!(enrolments -> courses (course));
diesel::joinable!(enrolments -> users (student));
diesel::joinable!(grade_assignments -> courses (course));
diesel::joinable!(grade_assignments_progress -> grade_assignments (assignment));
diesel::joinable!(grade_assignments_progress -> users (student));
diesel::joinable!(grade_assignments_revisions -> grade_assignments (id));
diesel::joinable!(indicies -> generations (generation));
diesel::joinable!(indicies -> programs (program));
diesel::joinable!(indicies -> users (student));
diesel::joinable!(point_assignments -> courses (course));
diesel::joinable!(point_assignments_progress -> point_assignments (assignment));
diesel::joinable!(point_assignments_progress -> users (student));
diesel::joinable!(point_assignments_revisions -> point_assignments (id));
diesel::joinable!(sessions -> users (user));
diesel::joinable!(users_revisions -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    courses_revisions,
    enrolments,
    generations,
    grade_assignments,
    grade_assignments_progress,
    grade_assignments_progress_revisions,
    grade_assignments_revisions,
    indicies,
    point_assignments,
    point_assignments_progress,
    point_assignments_progress_revisions,
    point_assignments_revisions,
    programs,
    sessions,
    users,
    users_revisions,
);
