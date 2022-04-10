use std::time::{SystemTime, UNIX_EPOCH};
extern crate redis;
use redis::Commands;
use std::sync::Arc;
use std::collections::*;
use parking_lot::RwLock;
use poem::{
    handler,
    http::{header, StatusCode},
    web::{
        websocket::{Message, WebSocket},
        Data, Json,
    },
    IntoResponse, Request, Response,
};

pub struct ConnectionManager {
    pub id: String,
    pub sessions: HashMap<String, HashMap<usize,tokio::sync::watch::Sender<String>>>//tokio::sync::broadcast::Sender<String>>,
    //red_client: redis::Client,
    //subscribers set
}

impl ConnectionManager {
    pub fn new(redis_host: String, redis_port: i16) -> Arc<RwLock<ConnectionManager>> {
        let red_client = redis::Client::open(format!("redis://{}:{}/", redis_host, redis_port)).unwrap();
        let connection_manager = ConnectionManager { id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_string(), sessions: HashMap::new() };
        let connection_manager = Arc::new(RwLock::new(connection_manager));
        let connection_manager_clone = Arc::clone(&connection_manager);
        let mut red = red_client.get_connection().unwrap();
        let mut red_pub_sub = red_client.get_connection().unwrap();
        tokio::spawn(async move  {
            let mut red_pub_sub = red_pub_sub.as_pubsub();
            red_pub_sub.subscribe("channel_1").unwrap();
            loop {
                let msg = red_pub_sub.get_message().unwrap();
                let payload : String = msg.get_payload().unwrap();
                println!("channel '{}': {}", msg.get_channel_name(), payload);
                let connection_manager = connection_manager.read(); 
                connection_manager.write_id_in_redis(&mut red);
                // connectionmanager send to person 
            }
        });
        connection_manager_clone
    }

    pub fn connect(&mut self, username: String, sender: tokio::sync::watch::Sender<String>/* set of subscribers*/) -> usize{
        let pos: usize; 
        match self.sessions.get_mut(&username) {
            Some(sockets) => {
                pos = sockets.len();
                sockets.insert(pos, sender);
                //debug
                println!("CONNECTED. username: {}, sockets: {}", &username, sockets.len());
            },
            None => {
                
                let mut new_sockets: HashMap<usize,tokio::sync::watch::Sender<String>> = HashMap::new();
                pos = 0;
                new_sockets.insert(0, sender);
                //debug
                println!("CONNECTED. username: {}, sessions: {}", &username, new_sockets.len());
                //end debug
                self.sessions.insert(username, new_sockets);
            }
        }
        // add server to user
        // notify subs
        return pos;
    }

    pub fn disconnect(&mut self, username: String, pos: usize){
        match self.sessions.get_mut(&username) {
            Some(sockets) => {
                sockets.remove(&pos);
                //debug
                println!("DISCONNECTED. username: {}, sessions: {}", &username, sockets.len());
            },
            None => ()
        }
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
        let _ : () = red.set("id", &self.id).unwrap();
    }

    fn test_redis_with_id(&self, red: &mut redis::Connection){
        let _ : () = red.set("id", &self.id).unwrap();
    }

    fn test_redis(red: &mut redis::Connection) {
        let _ : () = red.set("my_key", 42).unwrap();
    }

}

