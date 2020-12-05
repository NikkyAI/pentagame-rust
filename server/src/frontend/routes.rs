// imports
use super::errors::UserError;
use super::helper::check_password;
use super::{forms, templates};
use crate::auth::{generate_hash, guard_user, guard_with_user, verify_hash};
use crate::db::actions::{
    check_game, create_game, create_toast, create_user, get_cached_games, get_game, get_user_by_id,
    get_user_by_username, get_user_game, join_game, leave_game,
};
use crate::db::helper::zero_trim;
use crate::db::model::SlimUser;
use actix_identity::Identity;
use actix_web::error::ErrorBadRequest;
use actix_web::{
    dev::HttpResponseBuilder, dev::Payload, http::header, http::StatusCode, web::block, web::Data,
    web::Form, web::Path, Error, FromRequest, HttpRequest, HttpResponse,
};
use askama_actix::TemplateIntoResponse;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use futures::future::{err, ok, Ready};
use serde::Serialize;
use serde_json::from_str;
use uuid::Uuid;

// types
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type UserResponse = Result<HttpResponse, UserError>;

// Simple Redirect
pub fn redirect<'a>(route: &'a str) -> HttpResponse {
    HttpResponseBuilder::new(StatusCode::SEE_OTHER)
        .header(header::LOCATION, route)
        .finish()
}

// implementation of FromRequest Trait to allow for Guarded Routes
impl FromRequest for SlimUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = from_str(&user_json) {
                    return ok(user);
                };
            }
        }
        return err(ErrorBadRequest("No user identifiable"));
    }
}

// empty string constant
const EMPTY: &'static str = "";

/*
General API Responses
    ActionStatus:
        code:
            0: Success
            1: Failure
            2: invalid
            3: unauthorized
        description: Description of err/ success

    QueryResult:
        code:
            Same as ActionStatus
        data:
            Post (id, title, body, published)
*/

#[derive(Serialize)]
pub struct ActionStatus {
    code: i8,
    description: String,
}

#[derive(Serialize)]
pub struct QueryResult {
    code: i8,
    data: (u32, String, String, bool),
}

/*
General:
/ -> get_index
/cookies -> get_cookie_information

Not registered -> get_error_404
*/

pub async fn get_index(id: Option<SlimUser>) -> UserResponse {
    UserError::wrap_template(templates::IndexTemplate { id }.into_response())
}

pub async fn get_rules(id: Option<SlimUser>) -> UserResponse {
    UserError::wrap_template(templates::RulesTemplate { id }.into_response())
}

pub async fn get_cookies(id: Option<SlimUser>) -> UserResponse {
    UserError::wrap_template(templates::CookiesTemplate { id }.into_response())
}

pub async fn get_error_404(id: Option<SlimUser>) -> UserResponse {
    UserError::wrap_template(
        templates::ErrorTemplate {
            code: 404,
            id,
            message: "The requested page is not available".to_owned(),
        }
        .into_response(),
    )
}

/*
INFO: All routes except overview require are guarded

/games:
    /: get_game_overview -> Overview of current games and your profile
    /create: get_create_game -> Simple form for creating a new game
    /view/{id}: get_view_game -> View of game and it's participants
    /join/{id}: get_join_game -> Make user join game and redirect to game 'playing' screen
    /leave: Leave a game (a player may only join one game at a time. Can be changed anytime but works as architectural rate limiting)
*/

pub async fn get_game_join(
    id: Option<SlimUser>,
    path: Path<(i32,)>,
    pool: Data<DbPool>,
) -> UserResponse {
    // retrieve id and guard route
    let conn = pool.get()?;
    let uid = guard_with_user(id)?;

    let gid = block(move || check_game(&conn, path.0 .0)).await?;

    let conn = pool.get()?;

    // check if user already joined game
    let sacrifice = uid.id.clone();
    match block(move || get_user_game(&conn, sacrifice)).await? {
        Some(current_game_id) => {
            if current_game_id != gid {
                let sacrifice = uid.id.clone();
                let conn = pool.get()?;
                block(move || leave_game(&conn, sacrifice)).await?;
                let conn = pool.get()?;
                block(move || join_game(&conn, sacrifice, gid)).await?;
            }
        }
        None => {
            let conn = pool.get()?;
            block(move || join_game(&conn, sacrifice, gid)).await?;
        }
    }

    UserError::wrap_template(templates::GameBoardTemplate { id: Some(uid) }.into_response())
}

pub async fn get_game_overview(id: Option<SlimUser>, pool: Data<DbPool>) -> UserResponse {
    // acquire a connection
    let conn = pool.get()?;

    // this unfortunaly blocking due to the result limitations of the cache.
    //  Though this should only take a maximum of 2-8ms when building first time and even less when hitting cache (20s lifetime)
    let games = get_cached_games(&conn)?;

    UserError::wrap_template(templates::GamesOverviewTemplate { id, games }.into_response())
}

pub async fn get_create_game(id: Option<SlimUser>) -> UserResponse {
    UserError::wrap_template(
        templates::GamesCreateTemplate {
            id,
            name: EMPTY.to_owned(),
            description: EMPTY.to_owned(),
            name_error: false,
            description_error: false,
        }
        .into_response(),
    )
}

pub async fn post_create_game(
    data: Form<forms::GameForm>,
    id: Option<SlimUser>,
    pool: Data<DbPool>,
) -> UserResponse {
    // retrieve id and guard route
    let user = guard_with_user(id.clone())?;
    let conn = pool.get()?;

    // validates cookie checkbox
    let public = match &data.public {
        Some(content) => content == "on",
        None => true,
    };

    // freeing thread because diesel doesn't support async net
    let gid = block(move || {
        create_game(
            &conn,
            data.name.clone(),
            data.description.clone(),
            public,
            &user,
        )
    })
    .await?;

    Ok(redirect(&format!("/games/view/{}", gid)))
}

pub async fn get_view_game(
    path: Path<(i32,)>,
    id: Option<SlimUser>,
    pool: Data<DbPool>,
) -> UserResponse {
    guard_user(&id)?;
    let conn = pool.get()?;
    let gid = path.into_inner().0;

    let gdata = block(move || get_game(&conn, gid)).await?;

    let is_host = false;

    UserError::wrap_template(
        templates::GamesViewTemplate {
            id,
            is_host,
            game: gdata.0,
            players: gdata.1,
        }
        .into_response(),
    )
}

/*
Authentication & User managment
    users/login [GET|POST] -> users_login
        GET ->  UserLoginTemplate
        POST -> IndexTemplate || Referrer url
    users/register [GET] -> UserRegisterTemplate
    users/logout [GET] -> redirects to either home, referrer or login
    users/view/{id} [GET] (requires auth) -> UserViewTemplate
*/

pub async fn get_users_view(
    id: Option<SlimUser>,
    path: Path<Uuid>,
    pool: Data<DbPool>,
) -> UserResponse {
    guard_user(&id)?;
    let uid = path.0;
    let conn = pool.get()?;
    let user = block(move || get_user_by_id(&conn, uid)).await?;

    match user {
        Some(user) => {
            UserError::wrap_template(templates::UserViewTemplate { user, id }.into_response())
        }
        None => Err(UserError::NotFoundError()),
    }
}

pub async fn get_users_login(id: Option<SlimUser>) -> UserResponse {
    UserError::wrap_template(
        templates::UserLoginTemplate {
            username: "".to_owned(),
            password: "".to_owned(),
            cookie_error: false,
            username_error: false,
            id,
        }
        .into_response(),
    )
}

pub async fn post_users_login(
    id: Identity,
    pool: Data<DbPool>,
    form: Form<forms::UserForm>, // happens to have the required fields
) -> UserResponse {
    // validates cookie checkbox
    let cookie_error = match &form.cookies {
        Some(content) => content != "on",
        None => true,
    };

    // The auth system is based on cookies and you can't login without getting one
    if cookie_error {
        return UserError::wrap_template(
            templates::UserLoginTemplate {
                username: form.username.clone(),
                password: form.password.clone(),
                username_error: false,
                cookie_error,
                id: None,
            }
            .into_response(),
        );
    }

    // acquiring connection from db pool
    let conn = pool.get()?;

    /*
    I may expand the below part with fake hashing for time attack circumvention
    */
    let sacrifice = form.username.clone();
    let result = block(move || get_user_by_username(&conn, sacrifice)).await?;

    let user = match result {
        Some(user) => user,
        None => {
            return UserError::wrap_template(
                templates::UserLoginTemplate {
                    username: form.username.clone(),
                    password: form.password.clone(),
                    username_error: true,
                    cookie_error,
                    id: None,
                }
                .into_response(),
            );
        }
    };

    // verify hash and remember serialized Slimuser
    if verify_hash(&user.password, &form.password) {
        let user_string = serde_json::to_string(&SlimUser::from(user)).unwrap();
        id.remember(user_string);
    } else {
        return UserError::wrap_template(
            templates::UserLoginTemplate {
                username: form.username.clone(),
                password: form.password.clone(),
                username_error: true,
                cookie_error,
                id: None,
            }
            .into_response(),
        );
    }

    // redirect to get_index after done
    // TODO: ADD Alert option for index
    Ok(redirect("/"))
}

pub async fn get_logout_user(id: Identity) -> UserResponse {
    // Forgetting id means clearing auth cookie
    id.forget();
    // redirect to get_index after done
    // TODO: ADD Alert option for index

    Ok(redirect("/"))
}

pub async fn get_settings_user(id: Option<SlimUser>, pool: Data<DbPool>) -> UserResponse {
    let conn = pool.get()?;
    let identity = guard_with_user(id)?;

    let sacrifice = identity.username.clone();
    let result = block(move || get_user_by_username(&conn, sacrifice)).await?;

    /*
    for the unlikely case user session has outlived user in database
    When e.g. two sessions exist and one of them wasn't logged out on deletion
    */
    let user = match result {
        Some(user) => user,
        None => {
            return Err(UserError::ValidationError(format!(
                "User {} doesn't exist anymore or is archived",
                identity.username
            )));
        }
    };

    UserError::wrap_template(
        templates::UserSettingsTemplate {
            id: Some(identity),
            user,
            username_error: false,
            password_error: false,
            status_error: false,
        }
        .into_response(),
    )
}

pub async fn post_settings_user(
    id: Option<SlimUser>,
    pool: Data<DbPool>,
    data: Form<forms::SettingsForm>,
) -> UserResponse {
    let conn = pool.get()?;
    let identity = guard_with_user(id)?;

    println!(
        "username: {:?} password: {:?} new-password: {:?} status: {:?}",
        data.0.username, data.0.password, data.0.old_password, data.0.status
    );

    let sacrifice = identity.username.clone();
    let result = block(move || get_user_by_username(&conn, sacrifice)).await?;

    let mut user = match result {
        Some(user) => user,
        None => {
            return Err(UserError::ValidationError(format!(
                "User {} is deleted or achived",
                identity.username
            )));
        }
    };

    match data.0.password {
        Some(new) => match data.0.old_password {
            Some(old) => {
                if old == new || !check_password(&old) {
                    return UserError::wrap_template(
                        templates::UserSettingsTemplate {
                            user,
                            id: Some(identity),
                            status_error: false,
                            password_error: true,
                            username_error: false,
                        }
                        .into_response(),
                    );
                } else {
                    let sacrifice = user.id;
                    let conn = pool.get()?;
                    block(move || {
                        use crate::db::schema::users::dsl::*;
                        use diesel::{
                            query_dsl::filter_dsl::FilterDsl, update, ExpressionMethods,
                            RunQueryDsl,
                        };

                        update(users.filter(id.eq(&sacrifice)))
                            .set(password.eq(zero_trim(&generate_hash(old))))
                            .execute(&conn)?;

                        create_toast(
                            &conn,
                            sacrifice,
                            1_i16,
                            "Your password was changed".to_owned(),
                        )
                    })
                    .await?;

                    return UserError::wrap_template(
                        templates::UserSettingsTemplate {
                            user,
                            id: Some(identity),
                            status_error: false,
                            password_error: false,
                            username_error: false,
                        }
                        .into_response(),
                    );
                }
            }
            None => {
                return Err(UserError::ValidationError(
                    "Old Password not supplied. Invalid Form send".to_owned(),
                ));
            }
        },
        None => (),
    };

    match data.0.status {
        Some(new_status) => {
            let sacrifice = user.id;
            let sacrifice_status = new_status.clone();
            let conn = pool.get()?;
            block(move || {
                use crate::db::schema::users::dsl::*;
                use diesel::{
                    query_dsl::filter_dsl::FilterDsl, update, ExpressionMethods, RunQueryDsl,
                };

                update(users.filter(id.eq(&sacrifice)))
                    .set(status.eq(zero_trim(&sacrifice_status)))
                    .execute(&conn)?;

                create_toast(
                    &conn,
                    sacrifice,
                    1_i16,
                    "Your status was changed".to_owned(),
                )
            })
            .await?;

            user.status = new_status;

            return UserError::wrap_template(
                templates::UserSettingsTemplate {
                    user,
                    id: Some(identity),
                    status_error: false,
                    password_error: false,
                    username_error: false,
                }
                .into_response(),
            );
        }
        None => (),
    };

    match data.0.username {
        Some(new_username) => {
            let sacrifice_id = user.id;
            let sacrifice_new_username = new_username.clone();
            let conn = pool.get()?;
            match block(move || {
                use crate::db::schema::users::dsl::*;
                use diesel::{
                    query_dsl::filter_dsl::FilterDsl, update, ExpressionMethods, RunQueryDsl,
                };

                match get_user_by_username(&conn, sacrifice_new_username.clone())? {
                    Some(u) => {
                        println!("{}/{}", u.username, u.id);
                        Err(UserError::ValidationError("Username in use".to_owned()))
                    }
                    None => {
                        update(users.filter(id.eq(&sacrifice_id)))
                            .set(username.eq(zero_trim(&sacrifice_new_username)))
                            .execute(&conn)?;

                        Ok(create_toast(
                            &conn,
                            sacrifice_id,
                            1_i16,
                            "Your username was changed".to_owned(),
                        )?)
                    }
                }
            })
            .await
            {
                Ok(_) => {
                    user.username = new_username;
                    return UserError::wrap_template(
                        templates::UserSettingsTemplate {
                            user,
                            id: Some(identity),
                            status_error: false,
                            password_error: false,
                            username_error: false,
                        }
                        .into_response(),
                    );
                }
                Err(why) => {
                    eprintln!("{:?}", why);
                    return UserError::wrap_template(
                        templates::UserSettingsTemplate {
                            user,
                            id: Some(identity),
                            status_error: false,
                            password_error: false,
                            username_error: true,
                        }
                        .into_response(),
                    );
                }
            }
        }
        None => (),
    };

    return Err(UserError::InternalError(
        "Failed to respond appropriately".to_owned(),
    ));
}

pub async fn get_register_user() -> UserResponse {
    UserError::wrap_template(
        templates::UserRegisterTemplate {
            username: EMPTY.to_owned(),
            username_error: false,
            cookie_error: false,
            password_error: false,
            password: EMPTY.to_owned(),
            id: None,
            alert: EMPTY,
        }
        .into_response(),
    )
}

pub async fn post_register_user(
    id: Identity,
    pool: Data<DbPool>,
    form: Form<forms::UserForm>,
) -> UserResponse {
    // Validate fields
    let username_error = form.username.len() > 40_usize
        || form.username.len() < 1_usize
        || !form.username.is_ascii();
    let cookie_error = match &form.cookies {
        Some(content) => content != "on",
        None => true,
    };
    let password_error = check_password(&form.password);
    println!("{}/{}", form.password, password_error);
    if username_error || password_error || cookie_error {
        return UserError::wrap_template(
            templates::UserRegisterTemplate {
                username: form.username.clone(),
                username_error,
                password_error,
                cookie_error,
                password: form.password.clone(),
                id: None,
                alert: EMPTY,
            }
            .into_response(),
        );
    }

    // get connection from database pool
    let mut conn = pool.get()?;

    // to circumvent the `move` closure for web:block
    let username = form.username.clone();

    // check if username is already in use
    let user = block(move || get_user_by_username(&conn, username)).await?;

    match user {
        Some(_) => {
            return UserError::wrap_template(
                templates::UserRegisterTemplate {
                    username: form.username.clone(),
                    password: form.password.clone(),
                    id: None,
                    cookie_error: false,
                    alert: "Username already in use or reserved",
                    username_error: false,
                    password_error: false,
                }
                .into_response(),
            );
        }
        None => (),
    };

    // due to the `move` (and missing clone) requirement of web::block the connection needs to be reacquired
    conn = pool.get()?;

    let user = block(move || create_user(&conn, &form.username, &form.password)).await?;

    // logs new user in
    let user_string = serde_json::to_string(&user).unwrap();
    id.remember(user_string);

    // redirect to index
    Ok(redirect("/"))
}
