mod connection_manager;
mod file_reader;
mod functions;
mod models;

use mongodb::{bson::Document, Client};
use poem::{
    get, listener::TcpListener, middleware::CookieJarManager, post, EndpointExt, Result, Route,
    Server,
};

use poem::{
    handler,
    http::{header, StatusCode},
    web::{
        websocket::{Message, WebSocket},
        Data, Json, Html
    },
    IntoResponse, Request, Response,
};

use crate::connection_manager::ConnectionManager;
use crate::functions::routing;
use crate::models::*;


#[handler]
fn index() -> Html<&'static str> {
    Html(
        r###"
    <body>
        <form id="loginForm">
            Name: <input id="nameInput" type="text" />
            <button type="submit">Login</button>
        </form>
        
        <form id="sendForm" hidden>
            Text: <input id="msgInput" type="text" />
            <button type="submit">Send</button>
        </form>
        
        <textarea id="msgsArea" cols="50" rows="30" hidden></textarea>
    </body>
    <script>
        let ws;
        const loginForm = document.querySelector("#loginForm");
        const sendForm = document.querySelector("#sendForm");
        const nameInput = document.querySelector("#nameInput");
        const msgInput = document.querySelector("#msgInput");
        const msgsArea = document.querySelector("#msgsArea");
        
        nameInput.focus();
        loginForm.addEventListener("submit", function(event) {
            event.preventDefault();
            loginForm.hidden = true;
            sendForm.hidden = false;
            msgsArea.hidden = false;
            msgInput.focus();
            ws = new WebSocket("ws://127.0.0.1:5000/ws/" + nameInput.value);
            ws.onmessage = function(event) {
                msgsArea.value += event.data + "\r\n";
            }
        });
        
        sendForm.addEventListener("submit", function(event) {
            event.preventDefault();
            ws.send(msgInput.value);
            msgInput.value = "";
        });
    </script>
    "###,
    )
}



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
        .at("/", get(index))
        .at("/ws/:name", get(routing::ws))
        .at("/hello", get(routing::hello))
        .with(CookieJarManager::new())
        .data(connection_manager)
        .data(auth)
        .data(collections);

    Server::new(TcpListener::bind("127.0.0.1:5000"))
        .run(app)
        .await
}
