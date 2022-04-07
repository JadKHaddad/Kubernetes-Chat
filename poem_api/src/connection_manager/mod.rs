use std::time::{SystemTime, UNIX_EPOCH};
extern crate redis;
use redis::Commands;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct ConnectionManager {
    pub id: String,
    //red_client: redis::Client,
}

impl ConnectionManager {
    pub fn new(redis_host: String, redis_port: i16) -> Arc<RwLock<ConnectionManager>> {
        let red_client = redis::Client::open(format!("redis://{}:{}/", redis_host, redis_port)).unwrap();
        let connection_manager = ConnectionManager { id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_string()};
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

