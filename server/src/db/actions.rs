// imports
use super::helper::zero_trim;
use super::model::{Game, NewGame, NewUserGame, SlimUser, User, UserGame};
use super::schema::users;
use crate::auth::generate_hash;
use cached::{proc_macro::cached, stores::TimedCache};
use chrono::offset::Local;
use diesel::{
    delete, insert_into, result::Error, BelongingToDsl, ExpressionMethods, JoinOnDsl,
    OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
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

pub fn check_game(conn: &PgConnection, id: i32) -> Result<i32, Error> {
    use super::schema::games;

    games::table
        .find(id)
        .select(games::dsl::id)
        .first::<i32>(conn)
}

/*
TODO: Fix this caching at some point
#[cached(
    convert = "{ id }",
    type = "TimedCache<i32, (Game, Vec<(Uuid, String)>)>",
    result = true,
    create = "{ TimedCache::with_lifespan(30) }"
)]
*/
pub fn get_game(conn: &PgConnection, id: i32) -> Result<(Game, Vec<(Uuid, String)>), Error> {
    use super::schema::games;
    use super::schema::users::{dsl::id as uid, dsl::username};

    let game = games::table.find(id).first::<Game>(conn).optional()?;

    match game {
        Some(game) => {
            let users = UserGame::belonging_to(&game)
                .inner_join(users::table)
                .select((uid, username))
                .load::<(Uuid, String)>(conn)?;
            return Ok((game, users));
        }
        None => {
            return Err(Error::NotFound {});
        }
    }
}

#[cached(
    convert = "{ gid }",
    type = "TimedCache<i32, (String, Option<String>, i32)>",
    result = true,
    key = "i32",
    create = "{ TimedCache::with_lifespan(10) }"
)]
pub fn get_slim_game(
    conn: &PgConnection,
    gid: i32,
) -> Result<(String, Option<String>, i32), Error> {
    use super::schema::games::{self, dsl::*};

    games::table
        .find(gid)
        .select((name, description, id))
        .first::<(String, Option<String>, i32)>(conn)
}

#[cached(
    convert = "{ id }",
    type = "TimedCache<i32, Vec<(Uuid, String)>>",
    result = true,
    key = "i32",
    create = "{ TimedCache::with_lifespan(10) }"
)]
pub fn get_game_users(conn: &PgConnection, id: i32) -> Result<Vec<(Uuid, String)>, Error> {
    use super::schema::games;
    use super::schema::users::{dsl::id as uid, dsl::username};

    let game = games::table.find(id).first::<Game>(conn)?;
    let users = UserGame::belonging_to(&game)
        .inner_join(users::table)
        .select((uid, username))
        .load::<(Uuid, String)>(conn)?;
    return Ok(users);
}

#[cached(
    convert = "{ uid }",
    type = "TimedCache<Uuid, Vec<(i16, String)>>",
    result = true,
    key = "i32",
    create = "{ TimedCache::with_lifespan(30) }"
)]
pub fn get_user_alerts(conn: &PgConnection, uid: Uuid) -> Result<Vec<(i16, String)>, Error> {
    use super::schema::alerts::{self as s_alerts, dsl::*};

    let mut removable: Vec<i32> = Vec::new();
    let results = s_alerts::table
        .filter(user_id.eq(uid))
        .select((id, header_type, message))
        .load::<(i32, i16, String)>(conn)?;

    let user_alerts = results
        .iter()
        .map(|(alert_id, alert_type, alert_message)| {
            removable.push(*alert_id);
            (*alert_type, alert_message.clone())
        })
        .collect::<Vec<(i16, String)>>();

    delete(alerts.filter(id.eq_any(removable))).execute(conn)?;

    return Ok(user_alerts);
}

pub fn create_user(
    conn: &PgConnection,
    new_username: &String,
    new_password: &String,
) -> Result<SlimUser, Error> {
    use crate::db::schema::users::{id, username};

    let now = Local::now().naive_local();
    let status = format!("Player joined {:?}", now.date());
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

#[cached(
    convert = "{ name.clone() }",
    type = "TimedCache<String, Option<User>>",
    result = true,
    create = "{ TimedCache::with_lifespan(30) }"
)]
pub fn get_user_by_username(conn: &PgConnection, name: String) -> Result<Option<User>, Error> {
    use super::schema::users::dsl::*;

    users.filter(username.eq(&name)).first(conn).optional()
}

#[cached(
    convert = "{ uid }",
    type = "TimedCache<Uuid, Option<User>>",
    result = true,
    create = "{ TimedCache::with_lifespan(30) }"
)]
pub fn get_user_by_id(conn: &PgConnection, uid: Uuid) -> Result<Option<User>, Error> {
    use super::schema::users::dsl::*;

    users.filter(id.eq(uid)).first(conn).optional()
}

#[cached(
    convert = "{ uid }",
    type = "TimedCache<Uuid, Option<i32>>",
    key = "Uuid",
    result = true,
    create = "{ TimedCache::with_lifespan(30) }"
)]
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

pub fn leave_game(conn: &PgConnection, uid: Uuid) -> Result<usize, Error> {
    use super::schema::game_moves::{self, dsl::*};

    let subquery = game_moves::table
        .filter(user_id.eq(&uid))
        .select(id)
        .into_boxed();

    delete(
        game_moves::table
            .filter(user_id.eq(&uid))
            .filter(id.ne_all(subquery)),
    )
    .execute(conn)
}

pub fn join_game(conn: &PgConnection, user_id: Uuid, game_id: i32) -> Result<(), Error> {
    use super::schema::user_games;

    let new_user_game = NewUserGame { game_id, user_id };

    insert_into(user_games::table)
        .values(&new_user_game)
        .execute(conn)?;

    Ok(())
}

// WARNING: This can't use Result because of cached traits
// To resolve the key limitations of this store the 'fake_key' is used
// This may just panic a whole thread. DON'T USE THIS AGAINST AN UNKNOWN DATABASE
#[cached(
    type = "TimedCache<String, Vec<(i32, String)>>",
    result = true,
    create = "{ TimedCache::with_lifespan(30) }",
    convert = r#"{ "Keyless".to_owned() }"#
)]
pub fn get_cached_games(conn: &PgConnection) -> Result<Vec<(i32, String)>, Error> {
    use super::schema::games::dsl::*;

    games
        .select((id, name))
        .order(id.desc())
        .limit(5)
        .load::<(i32, String)>(conn)
}

pub fn create_toast(
    conn: &PgConnection,
    uid: Uuid,
    htype: i16,
    message: String,
) -> Result<(), Error> {
    use super::schema::alerts::dsl::{alerts, header_type, message as alert_message, user_id};

    insert_into(alerts)
        .values((
            header_type.eq(htype),
            user_id.eq(uid),
            alert_message.eq(zero_trim(&message)),
        ))
        .execute(conn)?;

    Ok(())
}
