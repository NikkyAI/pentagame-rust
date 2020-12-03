use super::actor::{Connect, Disconnect, GameServer, Message, QueryGameMessage, QueryUsersMessage};
use super::errors::{MESSAGE_FORMAT_ERROR, UNIMPLEMENTED_ERROR};
use crate::db::model::SlimUser;
use actix::prelude::*;
use actix_web_actors::ws;
use hashbrown::HashMap;
use serde::Serialize;
use serde_json::to_string;
use std::time::{Duration, Instant};

// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Serialize, Clone, Debug)]
struct ServerMessage {
    pub action: u8,
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Clone, Debug)]
struct ServerListMessage<T> {
    pub action: u8,
    pub data: Vec<T>,
}

pub struct WsGameSession {
    // unique session id (== user id)
    pub id: usize,
    // Client must send ping at least once per 30 seconds (CLIENT_TIMEOUT),
    // otherwise we drop connection.
    pub hb: Instant,
    // joined game
    pub game: i32,
    // Game server
    pub addr: Addr<GameServer>,
    // axtix identity bound
    pub uid: SlimUser,
}

impl Actor for WsGameSession {
    type Context = ws::WebsocketContext<Self>;

    // Method is called on actor start.
    // We register ws session with GameServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register address for server actor
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
                uid: self.uid.id,
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {
                        act.id =
                            res.expect("The game server failed to acquire a session fatal error");
                    }
                    // something is wrong with game server
                    Err(why) => {
                        eprintln!("The gamserver crashed: {:?}", why);
                        ctx.stop()
                    }
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify game server
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

// Handle messages from game server, we simply send it to peer websocket
impl Handler<Message> for WsGameSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(to_string(&msg).expect("The GameServer sends corrupt messages"));
    }
}

// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(why) => {
                eprintln!("Error: {:?}", why);
                return ctx.stop();
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                match serde_json::from_str::<Message>(&text) {
                    Ok(action) => {
                        // see action mapping in actors::ClientMessage
                        match action.action {
                            0 => {
                                self.addr.send(QueryUsersMessage { gid: self.game })
                            .into_actor(self)
                            .then(|res, _, ctx| {
                                let _ = match res {
                                     Ok(result) => {
                                        let users = match result {
                                                Ok(users) => users,
                                                Err(_) => {
                                                    ctx.stop();
                                                    return fut::ready(());
                                                }
                                        };
                                        let message = ServerListMessage {
                                            action: 0,
                                            data: users
                                        };

                                        let data = match serde_json::to_string(&message) {
                                                Ok(data) => data,
                                                Err(_) => {
                                                    ctx.stop();
                                                    return fut::ready(());
                                                }
                                        };

                                        ctx.text(data);

                                    }
                                    // something is wrong with game server
                                    Err(why) => {
                                        eprintln!("The gamserver crashed or game was closed: {:?}", why);
                                        ctx.stop()
                                    }
                                };
                                fut::ready(())

                            })
                            .wait(ctx);
                            }
                            1 => {
                                self.addr.send(QueryGameMessage { gid: self.game })
                            .into_actor(self)
                            // Result<(String, String, i32), APIError>
                            .then(|res, _, ctx| {
                                let _ = match res {
                                     Ok(result) => {
                                        let (name, description, status) = match result {
                                                Ok(users) => users,
                                                Err(_) => {
                                                    ctx.stop();
                                                    return fut::ready(());
                                                }
                                        };

                                        let mut data = HashMap::with_capacity(3);
                                        data.insert("name".to_owned(), name);
                                        data.insert("description".to_owned(), description);
                                        data.insert("status".to_owned(), status.to_string());
                                        let message = ServerMessage {
                                            action: 1,
                                             data
                                        };

                                        let data = match serde_json::to_string(&message) {
                                                Ok(data) => data,
                                                Err(_) => {
                                                    ctx.stop();
                                                    return fut::ready(());
                                                }
                                        };

                                        ctx.text(data);

                                    }
                                    // something is wrong with game server
                                    Err(why) => {
                                        eprintln!("The gamserver crashed or game was closed: {:?}", why);
                                        ctx.stop()
                                    }
                                };
                                fut::ready(())

                            })
                            .wait(ctx);
                            }
                            _ => ctx.text(UNIMPLEMENTED_ERROR.clone()),
                        };
                    }
                    Err(_) => ctx.text(MESSAGE_FORMAT_ERROR.clone()),
                }
            }
            ws::Message::Binary(_) => ctx.text(UNIMPLEMENTED_ERROR.clone()),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WsGameSession {
    // helper method that sends ping to client every second.
    //
    // also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify game server
                act.addr.do_send(Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}
