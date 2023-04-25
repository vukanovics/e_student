// @generated automatically by Diesel CLI.

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
