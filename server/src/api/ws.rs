use super::errors::APIError;
use crate::config::{DatabaseConfig, CONFIG};
use crate::db::actions::get_user_game;
use crate::frontend::routes::DbPool;
use actix::prelude::*;
use hashbrown::{HashMap, HashSet};
use rand::{self, rngs::ThreadRng, Rng};
use serde::Serialize;

// Game server sends this messages to session
#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct Message {
    /*
    | action | description                  | data             |
    | ------ | ---------------------------- | ---------------- |
    | 0      | {user} joined room           | {"user": {user}} |
    | 1      | {user} made move             | {                |
    |        |                              |  "user": {user}, |
    |        |                              |  "move": String  |
    |        |                              | }                |
    | 2      | {user} needs to place figure | {"user": {user}} |
    | 3      | {user} placed figure         | {                |
    |        |                              |  "user": {user}, |
    |        |                              |  "move": String  |
    |        |                              | }                |
    | 4      | {user} disocnnected          | {"user": {user}} |
    */
    pub action: u8,
    pub data: HashMap<String, String>,
}

// Message for game server communications

// New game session is created
#[derive(Message)]
#[rtype(result = "Result<usize, APIError>")]
pub struct Connect {
    // session id (== user id)
    pub addr: Recipient<Message>,
}

// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

// Send message to specific game
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    // Id of the client session
    pub id: usize,
    /*
    action & data
    ---
    | action | description         | data                | host only |
    | ------ | ------------------- | ----------------    | --------- |
    | 0      | get users           | {}                  |     X     |
    | 1      | make move           | {"move": String}    |     X     |
    | 2      | Place Stopper       | {"move": String}    |     X     |
    | 3      | leave game          | {}                  |     X     |
    | 4      | start game          | {"message": String} |     ✓     |
    | 5      | stop game           | {"message": String} |     ✓     |
    */
    pub action: u8,
    pub data: HashMap<String, String>,
    // Game id (this may be used to reference rooms more easyl)
    pub game: i32,
}

// Join Game
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    // Client id
    pub id: usize,
    // Game id
    pub game: i32,
}

// `GameServer` manages  and responsible for coordinating game sessions
pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    games: HashMap<i32, HashSet<usize>>,
    pool: DbPool,
    rng: ThreadRng,
}

impl Default for GameServer {
    fn default() -> GameServer {
        println!("Triggered defualt crteation");
        GameServer {
            games: HashMap::new(),
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

// Handler for Message message.
impl Handler<ClientMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        match msg.action {
            1 => (),
            _ => (),
        }
    }
}

// Join room, send disconnect message to old room
// send join message to new room
impl Handler<Join> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, game } = msg;
        let mut games = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.games {
            if sessions.remove(&id) {
                games.push(n.to_owned());
            }
        }

        // send message to other users
        let mut data = HashMap::with_capacity(1);
        data.insert(String::from("user"), msg.id.to_string());
        for game in games {
            self.send_message(&game, 3_u8, data.clone(), 0);
        }

        self.games
            .entry(game)
            .or_insert_with(HashSet::new)
            .insert(id);

        let mut data = HashMap::with_capacity(1);
        data.insert("user".to_owned(), id.to_string());
        self.send_message(&msg.game, 0, data, id);
    }
}
