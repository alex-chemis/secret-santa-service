// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        is_close -> Bool,
    }
}

diesel::table! {
    members (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
        is_admin -> Bool,
    }
}

diesel::table! {
    santas (id) {
        id -> Int4,
        group_id -> Int4,
        santa_id -> Int4,
        recipient_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(members -> groups (group_id));
diesel::joinable!(members -> users (user_id));
diesel::joinable!(santas -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    members,
    santas,
    users,
);
