use std::time::{SystemTime, UNIX_EPOCH};
extern crate redis;
use redis::Commands;
use std::future::Future;

pub struct ConnectionManager {
    id: String,
    red_client: redis::Client,
}

impl ConnectionManager {
    pub fn new(redis_host: String, redis_port: i16) -> Self {
        let red_client = redis::Client::open(format!("redis://{}:{}/", redis_host, redis_port)).unwrap();
        ConnectionManager { id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_string(), red_client: red_client}
    }

    pub fn init<'a>(&'a self){

        let mut red = self.red_client.get_connection().unwrap();
        let mut red_pub_sub = self.red_client.get_connection().unwrap();

        tokio::spawn(async move  {
            let mut red_pub_sub = red_pub_sub.as_pubsub();
            red_pub_sub.subscribe("channel_1").unwrap();
            loop {
                let msg = red_pub_sub.get_message().unwrap();
                let payload : String = msg.get_payload().unwrap();
                println!("channel '{}': {}", msg.get_channel_name(), payload);
                ConnectionManager::test_redis(&mut red);
                //self.test_redis_with_id(&mut red);
            }
        });

    }

    fn test_redis_with_id(&self, red: &mut redis::Connection){
        let _ : () = red.set("id", &self.id).unwrap();
    }

    fn test_redis(red: &mut redis::Connection) {
        let _ : () = red.set("my_key", 42).unwrap();
    }

}

