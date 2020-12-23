use std::convert::TryInto;

use crate::api::errors::APIError;
use crate::config::{DatabaseConfig, CONFIG};
use crate::db::actions::{
    fetch_latest_move, get_game, get_game_users, get_slim_game, get_user_game, make_new_move,
};
use crate::frontend::routes::DbPool;
use crate::graph::{graph::GraphState, graph::GRAPH, models::MOVE};
use actix::prelude::*;
use diesel::result::Error as DBError;
use hashbrown::{HashMap, HashSet};
use rand::{self, rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Game server sends this messages to session
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct Message {
    /*
    | action | description                  | data                 |
    | ------ | ---------------------------- | -------------------- |
    | 0      | {user} joined room           | {"user": {user}}     |
    | 1      | {user} made move             | {                    |
    |        |                              |  "user": {user},     |
    |        |                              |  "move": String      |
    |        |                              | }                    |
    | 2      | {user} needs to place figure | {"user": {user}}     |
    | 3      | {user} placed figure         | {                    |
    |        |                              |  "user": {user},     |
    |        |                              |  "move": String      |
    |        |                              | }                    |
    | 4      | {user} disconnected          | {"user": {user}}     |
    | 5      | Login                        | {                    |
    |        |                              |  "name": String,     |
    |        |                              |  "password": String, |
    |        |                              | }                    |

    Login is bound to websocket as cookie so no logout action required
    */
    pub action: u8,
    pub data: HashMap<String, String>,
}

// Message for game server communications

#[derive(Message)]
#[rtype(result = "Result<(String, String, i32, Vec<(Uuid, String)>), APIError>")]
pub struct QueryGameMessage {
    pub gid: i32,
}

#[derive(Message)]
#[rtype(result = "Result<bool, APIError>")]
pub struct MakeMoveMessage {
    // user id from game session
    pub uid: Uuid,
    // move to make/ validate
    pub action: MOVE,
    // related game id
    pub gid: i32,
}

// New game session is created
#[derive(Message)]
#[rtype(result = "Result<usize, APIError>")]
pub struct Connect {
    // session id (== user id)
    pub addr: Recipient<Message>,
    pub uid: Uuid,
}

// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

// Send message to specific game
#[derive(Message)]
#[rtype(result = "Result<(), APIError>")]
pub struct ClientMessage {
    // Id of the client session
    pub id: usize,
    /*
    action & data
    ---
    | action | description         | data                | host only |
    | ------ | ------------------- | ----------------    | --------- |
    | 1      | get game meta       | {}                  |     X     |
    | 2      | make move           | {"move": [MOVE]}    |     X     |
    | 3      | Place Stopper       | {"move": String}    |     X     |
    | 4      | leave game          | {}                  |     X     |
    | 5      | start game          | {"message": String} |     ✓     |
    | 6      | stop game           | {"message": String} |     ✓     |
    */
    pub action: u8,
    pub data: HashMap<String, String>,
    // Game id (this may be used to reference rooms more easyl)
    pub game: i32,
}

// `GameServer` manages  and responsible for coordinating game sessions
pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    games: HashMap<i32, HashSet<usize>>,
    states: HashMap<i32, GraphState>,
    pool: DbPool,
    rng: ThreadRng,
}

impl Default for GameServer {
    fn default() -> GameServer {
        println!("Triggered default creation");
        GameServer {
            games: HashMap::new(),
            states: HashMap::new(),
            sessions: HashMap::new(),
            pool: DatabaseConfig::init_pool(CONFIG.clone())
                .expect("Database pool failed to initialize"),
            rng: rand::thread_rng(),
        }
    }
}

impl GameServer {
    // Send message to all users in the room
    fn send_message(&self, game: &i32, action: u8, data: HashMap<String, String>, skip_id: usize) {
        if let Some(sessions) = self.games.get(game) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message {
                            action,
                            data: data.clone(),
                        });
                    }
                }
            }
        }
    }
}

// Make actor from `GameServer`
impl Actor for GameServer {
    // We are going to use simple Context, we just need ability to communicate
    // with other actors.
    type Context = Context<Self>;
}

// Handler for Connect message.
//
// Register new session and assign unique id to this session
impl Handler<Connect> for GameServer {
    type Result = Result<usize, APIError>;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // register session with random id. The +1 ensures that 0 is never a session id
        // to enable 0 as placeholder for nobody when skipping
        let id = self.rng.gen::<usize>() + 1_usize;
        self.sessions.insert(id, msg.addr);

        // add to group
        let conn = self.pool.get()?;
        let gid = match get_user_game(&conn, msg.uid)? {
            Some(id) => id,
            None => {
                return Err(APIError::ValidationError("Not joined any game".to_owned()));
            }
        };

        /*
        rebuilding state
        This should be cached (under normal circumstances) and not be too work
        */
        let state = match GraphState::build_from_db(&conn, gid) {
            Ok(state) => state,
            Err(_) => {
                return Err(APIError::InternalError(
                    "Unable to build graph state from database".to_owned(),
                ));
            }
        };

        match self.games.get_mut(&gid) {
            Some(game) => {
                game.insert(id);
                self.states.insert(gid, state);
            }
            None => {
                let mut new_game = HashSet::with_capacity(5);
                new_game.insert(id);
                self.games.insert(gid, new_game);
                self.states.insert(gid, state);
            }
        }

        // send id back
        Ok(id)
    }
}

// Handler for Disconnect message.
impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let mut games: Vec<i32> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.games {
                if sessions.remove(&msg.id) {
                    games.push(name.to_owned());
                }
            }
        }

        // send message to other users
        let mut data: HashMap<String, String> = HashMap::with_capacity(1);
        data.insert(String::from("user"), msg.id.to_string());
        for game in games {
            self.send_message(&game, 3_u8, data.clone(), 0);
        }
    }
}

// handler for user move
impl Handler<MakeMoveMessage> for GameServer {
    type Result = Result<bool, APIError>;

    fn handle(&mut self, msg: MakeMoveMessage, _: &mut Context<Self>) -> Self::Result {
        // get connections
        let conn = self.pool.get()?;
        let dest = [msg.action.0[3], msg.action.0[4], msg.action.0[5]];

        // fetch starting point (db is trusted source)
        let db_friendly_figure: i16 = msg.action.1.into(); // SMALLINT requires i16
        let src = match fetch_latest_move(&conn, msg.gid, msg.uid, db_friendly_figure) {
            // take response and translate to array
            Ok((action, _)) => {
                let (last_src, last_dest) = action.split_at(2);
                // ensure move isn't repetitive
                if dest == last_dest {
                    return Err(APIError::ValidationError(
                        "This move is repetitive".to_owned(),
                    ));
                } else {
                    last_src
                        .try_into()
                        .expect("Mov fetching returned corrupted results")
                }
            }
            // no move was made. Fall back
            Err(DBError::NotFound { .. }) => [db_friendly_figure, 0, 0],
            Err(_) => {
                eprintln!("Corrupted Database!!!!");
                return Err(APIError::InternalError(
                    "Potential Corrupted database!".to_owned(),
                ));
            }
        };

        // validate move
        let state = self.states.get(&msg.gid).unwrap();
        let mut graph = GRAPH.clone();
        graph.load_state(*state)?;
        let result = graph.validate(&src, &dest)?;

        match result.0 {
            true => {
                // add move to db
                make_new_move(
                    &conn,
                    msg.uid,
                    msg.gid,
                    (
                        [
                            *src.get(0).unwrap(),
                            *src.get(1).unwrap(),
                            *src.get(2).unwrap(),
                            dest[0],
                            dest[1],
                            dest[2],
                        ],
                        result.1,
                    ),
                )?;
                // send message of move to all other players

                Ok(true)
            }
            false => Err(APIError::ValidationError(
                "This move isn't possible".to_owned(),
            )),
        }
    }
}

// handler for game query message
impl Handler<QueryGameMessage> for GameServer {
    type Result = Result<(String, String, i32, Vec<(Uuid, String)>), APIError>;

    fn handle(&mut self, msg: QueryGameMessage, _: &mut Context<Self>) -> Self::Result {
        let conn = self.pool.get()?;

        let game = get_slim_game(&conn, msg.gid)?;
        let users = get_game_users(&conn, msg.gid)?;

        match game.1 {
            Some(desc) => Ok((game.0, desc, game.2, users)),
            None => Ok((game.0, "".to_owned(), game.2, users)),
        }
    }
}

// Handler for ClientMessage message.
impl Handler<ClientMessage> for GameServer {
    type Result = Result<(), APIError>;

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) -> Self::Result {
        match msg.action {
            0 => {
                let conn = self.pool.get()?;
                let (_, users) = get_game(&conn, msg.game)?;

                let mut data = HashMap::with_capacity(users.len());
                users.iter().for_each(|(id, name)| {
                    data.insert(id.to_string(), name.clone());
                });

                Ok(self.sessions.get(&msg.id).unwrap().do_send(Message {
                    action: msg.action,
                    data,
                })?)
            }
            1 => Ok(()),
            _ => Ok(()),
        }
    }
}

// Join room, send disconnect message to old room
// send join message to new room
/*
impl Handler<Join> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, game } = msg; // I love this on-the-fly syntax
        let mut games = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.games {
            if sessions.remove(&id) {
                games.push(n.to_owned());
            }
        }

        // insert into internal game map
        self.games
            .entry(game)
            .or_insert_with(HashSet::new)
            .insert(id);

        // ensure states are set
        /*
        match self.states.get(&game) {
            Some(_) => (),
            None => {
                self.states.insert(game, GRAPH)
            }
        }
        */

        let mut data = HashMap::with_capacity(1);
        data.insert("user".to_owned(), id.to_string());
        self.send_message(&msg.game, 0, data, id);
    }
}
**/