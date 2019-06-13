table! {
    access_tokens (token) {
        token -> Nullable<Text>,
        user_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    articles (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        title -> Text,
        body -> Text,
        accessible -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        encrypted_password -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

joinable!(access_tokens -> users (user_id));
joinable!(articles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    articles,
    users,
);
