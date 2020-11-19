// imports
use super::helper::zero_trim;
use super::model::{Game, NewGame, NewUserGame, SlimUser, User, UserGame};
use super::schema::users;
use crate::auth::generate_hash;
use cached::{proc_macro::cached, stores::TimedCache};
use chrono::offset::Local;
use diesel::{
    insert_into, result::Error, BelongingToDsl, ExpressionMethods, JoinOnDsl, OptionalExtension,
    PgConnection, QueryDsl, RunQueryDsl,
};
use uuid::Uuid;

pub fn create_game(
    conn: &PgConnection,
    name: String,
    description: Option<String>,
    public: bool,
    id: &SlimUser,
) -> Result<i32, Error> {
    use super::schema::{games, user_games};

    let new_description = match description {
        Some(content) => Some(zero_trim(&content)),
        None => None,
    };

    let new_game = NewGame {
        name: zero_trim(&name),
        description: new_description,
        user_id: id.id,
        public,
        state: 0, // see state mapping in server/db/models
    };

    let gid = insert_into(games::table)
        .values(&new_game)
        .returning(games::id)
        .get_result::<i32>(conn)?;

    let new_game_bind = NewUserGame {
        game_id: gid.clone(),
        user_id: id.id,
    };

    insert_into(user_games::table)
        .values(&new_game_bind)
        .execute(conn)?;

    return Ok(gid);
}

pub fn get_game(
    conn: &PgConnection,
    id: i32,
) -> Result<Option<(Game, Vec<(Uuid, String)>)>, Error> {
    use super::schema::games;
    use super::schema::users::{dsl::id as uid, dsl::username};

    let game = games::table.find(id).first::<Game>(conn).optional()?;

    match game {
        Some(game) => {
            let users = UserGame::belonging_to(&game)
                .inner_join(users::table)
                .select((uid, username))
                .load::<(Uuid, String)>(conn)
                .expect("Couldn't find any users. Corrupt Gamerecord");
            return Ok(Some((game, users)));
        }
        None => {
            return Ok(None);
        }
    }
}

pub fn create_user(
    conn: &PgConnection,
    new_username: &String,
    new_password: &String,
) -> Result<SlimUser, Error> {
    use crate::db::schema::users::{id, username};

    let now = Local::now().naive_local();
    let status = format!("Author joined {:?}", now.date());
    let hash = generate_hash(new_password.clone());

    let new_users = User {
        active: true,
        username: zero_trim(new_username),
        password: zero_trim(&hash),
        id: Uuid::new_v4(),
        created_at: now,
        status,
    };

    let res = insert_into(users::table)
        .values(&new_users)
        .returning((id, username))
        .get_result::<(Uuid, String)>(conn)?;

    Ok(SlimUser {
        id: res.0,
        username: res.1,
    })
}

pub fn get_user_by_username(conn: &PgConnection, name: String) -> Result<Option<User>, Error> {
    use super::schema::users::dsl::*;

    users.filter(username.eq(&name)).first(conn).optional()
}

pub fn get_user_game(conn: &PgConnection, uid: Uuid) -> Result<Option<i32>, Error> {
    use super::schema::games::{self, id as gid, state};
    use super::schema::user_games::{self, user_id};

    let user = users::table.find(uid).first::<User>(conn).optional()?;

    match user {
        Some(_) => Ok(games::table
            .inner_join(user_games::table.on(user_id.eq(uid)))
            .filter(state.lt(11))
            .select(gid)
            .first::<i32>(conn)
            .optional()?),
        None => Ok(None),
    }
}

// WARNING: This can't use Result because of cached traits
// To resolve the key limitations of this store the 'fake_key' is used
// This may just panic a whole thread. DON'T USE THIS AGAINST AN UNKNOWN DATABASE
#[cached(
    type = "TimedCache<String, Vec<(i32, String)>>",
    create = "{ TimedCache::with_lifespan(30) }",
    convert = r#"{ format!("{}", _fake_key) }"#
)]
pub fn get_cached_games(conn: &PgConnection, _fake_key: u8) -> Vec<(i32, String)> {
    use super::schema::games::dsl::*;

    games
        .select((id, name))
        .order(id.desc())
        .limit(5)
        .load::<(i32, String)>(conn)
        .expect("Couldn't load games for cache. CRITICAL ERROR")
}
