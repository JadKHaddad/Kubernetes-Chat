#[derive(Debug)]
pub struct ConnectionManager {
    id: String,
    redis_host: String,
    redis_port: i16,
}

impl ConnectionManager {
    pub fn new(redis_host: String, redis_port: i16) -> Self {
        ConnectionManager { id: String::from("new id"), redis_host, redis_port}
    }
}

