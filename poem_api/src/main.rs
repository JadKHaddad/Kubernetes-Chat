use futures_util::{SinkExt, StreamExt};
use parking_lot::RwLock;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};
use std::time::{SystemTime, UNIX_EPOCH};
use poem::{
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    middleware::{AddData, CookieJarManager},
    session::{CookieConfig, MemoryStorage, ServerSession, Session},
    web::{
        websocket::{Message, WebSocket},
        Data, Html, Path,
    },
    EndpointExt, IntoResponse, Request, Response, Result, Route, Server,
};
use std::sync::Arc;
mod file_reader;
mod connection_manager;
mod models;
use models::mongodb_models;
use connection_manager::ConnectionManager;

#[derive(Clone, Debug)]
struct st {
    count: i32,
}

impl st {
    pub fn inc(&mut self) {
        self.count += 1;
    }
}

#[handler]
async fn hello(
    req: &Request,
    s: Data<&Arc<RwLock<ConnectionManager>>>,
    auth: Data<&fireauth::FireAuth>,
) -> Response {
    let email = "something@email.com";
    let password = "supersecretji";
    let return_secure_token = true;



    match auth
        .sign_in_email(email, password, return_secure_token).await
    {
        Ok(response) => println!("{:?}", response),
        Err(error) => println!("{:?}", error),
    }
    let cookie_value: Option<String> = match req.cookie().get("cookie") {
        Some(cookie) => Some(String::from(cookie.value_str())),
        None => None,
    };
    match cookie_value {
        Some(cookie_value) => {
            println!("cookie: {}", cookie_value);
        }
        None => {
            println!("no cookie");
        }
    }
    let mut builder = Response::builder()
        .header("Set-Cookie", "cookie=wassap; SameSite=lax")
        .header("content-security-policy", "default-src 'self';base-uri 'self';block-all-mixed-content;font-src 'self' https: data:;form-action 'self';frame-ancestors 'self';img-src 'self' data:;object-src 'none';script-src 'self';script-src-attr 'none';style-src 'self' https: 'unsafe-inline';upgrade-insecure-requests")
        .status(StatusCode::OK);


    let mut w = s.write();
    println!("{}", w.id);
    w.id = "123".to_string();


    return builder.body("ok");
}

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
            ws = new WebSocket("ws://127.0.0.1:3000/ws/" + nameInput.value);
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

#[handler]
fn ws(
    //Path(name): Path<String>,
    ws: WebSocket,
    req: &Request,
    //sender: Data<&tokio::sync::broadcast::Sender<String>>,
) -> impl IntoResponse {
    //let sender = sender.clone();
    //let mut receiver = sender.subscribe();

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();

        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(text) = msg {
                    if sink
                        .send(Message::Text(format!("sent to self, {}", text)))
                        .await
                        .is_err()
                    {
                        break;
                    }
                    // if sender.send(format!("{}: {}", name, text)).is_err() {
                    //     break;
                    // }
                }
            }
            println!("disconnected");
        });

        // tokio::spawn(async move {
        //     while let Ok(msg) = receiver.recv().await {
        //         println!("2");
        //         if sink.send(Message::Text(msg)).await.is_err() {
        //             break;
        //         }
        //     }
        // });
    })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();



    let config = file_reader::get_config();
    let connection_manager = ConnectionManager::new(config.redis_host, config.redis_port);
    //connection_manager.init();


    let connection_manager = Arc::new(RwLock::new(connection_manager));
    let arc_num_clone = Arc::clone(&connection_manager);
    ConnectionManager::a(arc_num_clone);

    let firebase_config = file_reader::get_firebase_config();
    let auth = fireauth::FireAuth::new(firebase_config.api_key);


    let mongodb = Client::with_uri_str(format!("mongodb://{}:{}", config.mongo_host, config.mongo_port).as_str())
        .await
        .unwrap()
        .database(config.mongo_db_name.as_str());
    let users_collection = mongodb.collection::<Document>("Users");



    /*let result = users_collection
    .insert_one(
        doc! {
            "email": "sadasd",
            "username": "asd",
            "signupTimeStamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_string(),
            "contacts": [],
            "subscribers": [],
        },
        None,
    )
    .await
    .unwrap();
    */

    let app = Route::new()
        .at("/", get(index))
        .at(
            "/ws",
            get(ws), //.data(tokio::sync::broadcast::channel::<String>(32).0)),
        )
        .at("/hello", get(hello))
        .with(CookieJarManager::new())
        .data(connection_manager)
        .data(auth)
        .data(users_collection);

    Server::new(TcpListener::bind("127.0.0.1:5000"))
        .run(app)
        .await
}
