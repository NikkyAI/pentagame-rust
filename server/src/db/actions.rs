// imports
use super::helper::zero_trim;
use super::model::{Game, NewGame, NewGameMove, NewUserGame, SlimUser, User, UserGame};
use super::schema::users;
use crate::auth::generate_hash;
use crate::graph::models::MOVE;
use cached::{proc_macro::cached, stores::TimedCache};
use chrono::offset::Local;
use diesel::{
    delete, insert_into, result::Error, BelongingToDsl, ExpressionMethods, JoinOnDsl,
    OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
};
use std::convert::TryInto;
use uuid::Uuid;

pub fn create_game(
    conn: &PgConnection,
    name: String,
    description: Option<String>,
    public: bool,
    icon: String,
    id: &SlimUser,
) -> Result<i32, Error> {
    use super::schema::{games, user_games};
    // INFO: Data (including icon, …) is seen as validated/ trusted at this point

    let new_description = match description {
        Some(content) => Some(zero_trim(&content)),
        None => None,
    };

    let new_game = NewGame {
        name: zero_trim(&name),
        description: new_description,
        user_id: id.id,
        public,
        icon: zero_trim(&icon),
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

#[cached(
    convert = "{ gid }",
    type = "TimedCache<i32, (Game, Vec<(Uuid, String)>)>",
    result = true,
    key = "i32",
    create = "{ TimedCache::with_lifespan(30) }"
)]
pub fn get_game(conn: &PgConnection, gid: i32) -> Result<(Game, Vec<(Uuid, String)>), Error> {
    use super::schema::games;
    use super::schema::users::{dsl::id as uid, dsl::username};

    let game = games::table.find(gid).first::<Game>(conn).optional()?;

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

pub fn leave_game(conn: &PgConnection, uid: Uuid) -> Result<(), Error> {
    use super::schema::game_moves::{self, dsl::*};
    use super::schema::user_games::{self, dsl::id as user_game_id, dsl::user_id};

    /*
    -> get latest game id
        -> get latest user_games from all players -> take game_id and index
    -> get latest figure_id
        -> calculate figure ids -> remove all entries matching figure ids and game_id
    -> Return
    */

    let user_game = user_games::table
        .filter(user_id.eq(&uid))
        .select(user_games::all_columns)
        .order_by(user_game_id)
        .first::<UserGame>(conn)?;

    delete(user_games::table.filter(user_game_id.eq(user_game.id))).execute(conn)?;
    delete(game_moves::table.filter(game_id.eq(user_game.game_id))).execute(conn)?;

    Ok(())
}

pub fn join_game(conn: &PgConnection, user_id: Uuid, game_id: i32) -> Result<(), Error> {
    use super::schema::user_games;

    let new_user_game = NewUserGame { game_id, user_id };

    insert_into(user_games::table)
        .values(&new_user_game)
        .execute(conn)?;

    Ok(())
}

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

pub fn fetch_latest_move(conn: &PgConnection, gid: i32, uid: Uuid, fid: i16) -> Result<MOVE, Error> {
    use super::schema::game_moves::dsl::*;

    // the database will always return arrays with the size of 3 -> two arrays size 6
    let mut action = game_moves
        .select((src, dest, figure))
        .limit(1)
        .order(id.desc())
        .filter(user_id.eq(uid))
        .filter(game_id.eq(gid))
        .filter(figure.eq(fid))
        .first::<(Vec<i16>, Vec<i16>, i16)>(conn)?;

    // create new array to hold retrieved values
    let mut locations: [i16; 6] = [0_i16; 6];
    let mut index: usize = 0;

    // fill new array with content from database arrays
    action.0.drain(0..3).for_each(|location| {
        locations[index] = location;
        index += 1;
    });

    action.1.drain(3..6).for_each(|location| {
        locations[index] = location;
        index += 1;
    });

    Ok((
        locations,
        action
            .2
            .try_into()
            .expect("Corrupted Database entry for figure id"),
    ))
}

pub fn make_new_move(
    conn: &PgConnection,
    user_id: Uuid,
    game_id: i32,
    action: MOVE,
) -> Result<usize, Error> {
    use super::schema::game_moves;

    let (src, dest) = action.0.split_at(2);

    insert_into(game_moves::table)
        .values(NewGameMove {
            game_id,
            src,
            dest,
            user_id,
            figure: action.1.into(),
        })
        .execute(conn)
}
