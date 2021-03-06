use std::time::{SystemTime, UNIX_EPOCH};
extern crate redis;
use crate::models::ws_models;
use parking_lot::RwLock;
use redis::Commands;
use std::collections::*;
use std::sync::Arc;

pub struct User {
    pub positions: HashSet<usize>,
    pub sessions: HashMap<usize, tokio::sync::watch::Sender<String>>,
    pub subscribers: HashSet<String>,
}

pub struct ConnectionManager {
    pub id: String,
    pub sessions: HashMap<String, User>,
}

impl ConnectionManager {
    pub fn new(redis_host: String, redis_port: i16) -> Arc<RwLock<ConnectionManager>> {
        let red_client =
            redis::Client::open(format!("redis://{}:{}/", redis_host, redis_port)).unwrap();
        let connection_manager = ConnectionManager {
            id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros()
                .to_string(),
            sessions: HashMap::new(),
        };
        let connection_manager = Arc::new(RwLock::new(connection_manager));
        let connection_manager_clone = Arc::clone(&connection_manager);
        let mut red = red_client.get_connection().unwrap();
        let mut red_pub_sub = red_client.get_connection().unwrap();
        tokio::spawn(async move {
            let mut red_pub_sub = red_pub_sub.as_pubsub();
            red_pub_sub.subscribe("channel_1").unwrap();
            loop {
                let msg = red_pub_sub.get_message().unwrap();
                let payload: String = msg.get_payload().unwrap();
                println!("channel '{}': {}", msg.get_channel_name(), payload);
                let connection_manager = connection_manager.read();
                connection_manager.write_id_in_redis(&mut red);
                // connectionmanager send to person
            }
        });
        connection_manager_clone
    }

    pub fn connect(
        &mut self,
        username: String,
        subscribers:  HashSet<String>,
        sender: tokio::sync::watch::Sender<String>,
    ) -> usize {
        let mut pos: usize;
        match self.sessions.get_mut(&username) {
            Some(user) => {
                pos = 0;
                while user.positions.contains(&pos) {
                    pos += 1;
                }
                user.positions.insert(pos);
                user.sessions.insert(pos, sender);
                //debug
                println!(
                    "CONNECTED. username: {}, sockets: {}",
                    &username,
                    user.sessions.len()
                );
            }
            None => {
                pos = 0;
                let mut new_sessions: HashMap<usize, tokio::sync::watch::Sender<String>> =
                    HashMap::new();
                let mut new_positions: HashSet<usize> = HashSet::new();
                new_positions.insert(0);
                new_sessions.insert(0, sender);
                //debug
                println!(
                    "CONNECTED. username: {}, sessions: {}",
                    &username,
                    new_sessions.len()
                );
                //end debug
                self.sessions.insert(
                    username,
                    User {
                        positions: new_positions,
                        sessions: new_sessions,
                        subscribers: subscribers,
                    },
                );
            }
        }
        // add server to user
        // notify subs
        return pos;
    }

    pub fn disconnect(&mut self, username: String, pos: usize) {
        match self.sessions.get_mut(&username) {
            Some(user) => {
                user.sessions.remove(&pos);
                user.positions.remove(&pos);
                //debug
                println!(
                    "DISCONNECTED. username: {}, sessions: {}",
                    &username,
                    user.sessions.len()
                );
            }
            None => (),
        }
    }

    pub fn send_personal_message(
        &self,
        username_from: &str,
        username_to: &str,
        text_content: &str,
    ) {
        //send to self
        let message_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let self_message = ws_models::Message {
            event_type: "msg",
            event_content: ws_models::EventContent {
                username_target: username_to,
                message_content: ws_models::MessageContent {
                    r#type: "txt",
                    text_content: text_content,
                    date: "", // todo
                    id: &message_id,
                    received: false,
                },
            },
        };
        self.send(
            username_from,
            &serde_json::to_string(&self_message).unwrap(),
        );
        //notify!

        //send to other
        let others_message = ws_models::Message {
            event_type: "msg",
            event_content: ws_models::EventContent {
                username_target: username_from,
                message_content: ws_models::MessageContent {
                    r#type: "txt",
                    text_content: text_content,
                    date: "",
                    id: &message_id,
                    received: true,
                },
            },
        };
        self.send(
            username_to,
            &serde_json::to_string(&others_message).unwrap(),
        );
        //notify!
    }

    pub fn send_personal_typing(&self, username_from: &str, username_to: &str) {
        let typing_message = ws_models::TypingMessage {
            event_type: "typing",
            username_target: username_from,
        };
        self.send(
            username_to,
            &serde_json::to_string(&typing_message).unwrap(),
        );
    }

    fn send(&self, username_to: &str, content: &str) {
        if let Some(user) = self.sessions.get(username_to) {
            for (_, tx) in user.sessions.iter() {
                tx.send(String::from(content)).unwrap();
            }
        }
    }

    pub fn add_to_subscribers(&mut self, subscriber:  String, username_to:  &str){
        if let Some(user) = self.sessions.get_mut(username_to) {
            user.subscribers.insert(subscriber);
        }
        //notify
    }

    // pub fn a(m: Arc<RwLock<ConnectionManager>>){
    //     let mm = m.read();
    //     let mut red = mm.red_client.get_connection().unwrap();
    //     let mut red_pub_sub = mm.red_client.get_connection().unwrap();
    //     let arc_num_clone = Arc::clone(&m);
    //     tokio::spawn(async move  {
    //         let mut red_pub_sub = red_pub_sub.as_pubsub();
    //         red_pub_sub.subscribe("channel_1").unwrap();
    //         loop {
    //             let msg = red_pub_sub.get_message().unwrap();
    //             let payload : String = msg.get_payload().unwrap();
    //             println!("channel '{}': {}", msg.get_channel_name(), payload);
    //             ConnectionManager::test_redis(&mut red);
    //             let s = arc_num_clone.read();
    //             println!("{}", s.id);
    //             s.test_redis_with_id(&mut red);
    //         }
    //     });
    // }

    // pub fn init<'a>(&'a self){

    //     //let mut red = self.red_client.get_connection().unwrap();
    //     //let mut red_pub_sub = self.red_client.get_connection().unwrap();

    //     tokio::spawn(async move  {
    //         let mut red_pub_sub = red_pub_sub.as_pubsub();
    //         red_pub_sub.subscribe("channel_1").unwrap();
    //         loop {
    //             let msg = red_pub_sub.get_message().unwrap();
    //             let payload : String = msg.get_payload().unwrap();
    //             println!("channel '{}': {}", msg.get_channel_name(), payload);
    //             ConnectionManager::test_redis(&mut red);
    //             //self.test_redis_with_id(&mut red);
    //         }
    //     });

    // }
    fn write_id_in_redis(&self, red: &mut redis::Connection) {
        let _: () = red.set("id", &self.id).unwrap();
    }

    /*
    fn test_redis_with_id(&self, red: &mut redis::Connection) {
        let _: () = red.set("id", &self.id).unwrap();
    }

    fn test_redis(red: &mut redis::Connection) {
        let _: () = red.set("my_key", 42).unwrap();
    }
    */
}
