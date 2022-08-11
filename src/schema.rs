table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        // status -> Enum,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    user_tokens (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Varchar,
        expires_at -> Datetime,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        password -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    user_tokens,
    users,
);
