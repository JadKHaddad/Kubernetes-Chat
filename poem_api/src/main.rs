mod connection_manager;
mod file_reader;
mod functions;
mod models;

use mongodb::{bson::Document, Client};
use poem::{
    get, listener::TcpListener, middleware::CookieJarManager, post, EndpointExt, Result, Route,
    Server,
};

use crate::connection_manager::ConnectionManager;
use crate::functions::routing;
use crate::models::*;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let config = file_reader::get_config();
    let connection_manager = ConnectionManager::new(config.redis_host, config.redis_port);
    let firebase_config = file_reader::get_firebase_config();
    let auth = fireauth::FireAuth::new(firebase_config.api_key);
    let mongodb = Client::with_uri_str(
        format!("mongodb://{}:{}", config.mongo_host, config.mongo_port).as_str(),
    )
    .await
    .unwrap()
    .database(config.mongo_db_name.as_str());
    let users_collection = mongodb.collection::<Document>("Users");
    let sessions_collection = mongodb.collection::<Document>("Sessions");
    let collections = mongodb_models::Collections {
        users_collection: users_collection,
        sessions_collection: sessions_collection,
    };

    let app = Route::new()
        .at("/signup", post(routing::signup))
        .at("/signin", post(routing::signin))
        .at("/signout", post(routing::signout))
        .at("/isSignedin", post(routing::is_signedin))
        .at("/addContact", post(routing::add_contact))
        .at(
            "/ws",
            get(routing::ws), //.data(tokio::sync::broadcast::channel::<String>(32).0)),
        )
        .at("/hello", get(routing::hello))
        .with(CookieJarManager::new())
        .data(connection_manager)
        .data(auth)
        .data(collections);

    Server::new(TcpListener::bind("127.0.0.1:5000"))
        .run(app)
        .await
}
