table! {
    games (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        host_id -> Uuid,
    }
}

table! {
    user_games (id) {
        id -> Int4,
        player_id -> Uuid,
        game_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        active -> Bool,
        password -> Text,
        created_at -> Timestamp,
    }
}

joinable!(games -> users (host_id));
joinable!(user_games -> games (game_id));
joinable!(user_games -> users (player_id));

allow_tables_to_appear_in_same_query!(
    games,
    user_games,
    users,
);
