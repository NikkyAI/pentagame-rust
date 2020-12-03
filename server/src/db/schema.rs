table! {
    game_moves (id) {
        id -> Int4,
        user_id -> Uuid,
        game_id -> Int4,
        umove -> Array<Int4>,
    }
}

table! {
    games (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        user_id -> Uuid,
        state -> Int2,
        public -> Bool,
    }
}

table! {
    user_games (id) {
        id -> Int4,
        user_id -> Uuid,
        game_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        active -> Bool,
        password -> Text,
        status -> Text,
        created_at -> Timestamp,
    }
}

joinable!(game_moves -> games (game_id));
joinable!(game_moves -> users (user_id));
joinable!(games -> users (user_id));
joinable!(user_games -> games (game_id));
joinable!(user_games -> users (user_id));

allow_tables_to_appear_in_same_query!(
    game_moves,
    games,
    user_games,
    users,
);
