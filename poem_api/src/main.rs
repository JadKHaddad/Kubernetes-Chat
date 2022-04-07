use futures_util::{SinkExt, StreamExt};
use mongodb::{
    bson::{doc, Document},
    error::Error as MongoError,
    results::InsertOneResult,
    Client, Collection,
};
use parking_lot::RwLock;
use poem::{
    get, handler,
    http::{header, StatusCode},
    listener::TcpListener,
    middleware::CookieJarManager,
    post,
    web::{
        websocket::{Message, WebSocket},
        Data, Json,
    },
    EndpointExt, IntoResponse, Request, Response, Result, Route, Server,
};
use rand::{distributions::Alphanumeric, Rng};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

mod connection_manager;
mod file_reader;
mod models;

use connection_manager::ConnectionManager;
use models::*;

fn create_token() -> String {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    return token;
}

async fn create_session(
    username: &str,
    token: &str,
    sessoins_collection: &Collection<Document>,
) -> Result<InsertOneResult, MongoError> {
    let result = sessoins_collection
        .insert_one(
            doc! {
                "token": username,
                "username": token,
            },
            None,
        )
        .await?;
    return Ok(result);
}

#[handler]
async fn signup(
    req: Json<request_models::SignupModel>,
    auth: Data<&fireauth::FireAuth>,
    collections: Data<&mongodb_models::Collections>,
) -> Response {
    let mut builder = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK);
    let mut success = false;
    let mut message = String::from("username exists");
    match collections
        .users_collection
        .find_one(doc! {"username": &req.username}, None)
        .await
        .unwrap()
    {
        Some(_) => {
            //user exists
            let response = response_models::SignupModel {
                success: success,
                message: message,
            };
            return builder.body(serde_json::to_string(&response).unwrap());
        }
        None => (),
    }
    //create a new user
    match auth.sign_up_email(&req.email, &req.password, true).await {
        Ok(new_user) => {
            success = true;
            message = String::from("success");
            let _ = collections.users_collection
    .insert_one(
        doc! {
            "localId": &new_user.local_id,
            "email": &req.email,
            "username": &req.username,
            "signupTimeStamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros().to_string(),
            "contacts": [],
            "subscribers": [],
        },
        None,
    )
    .await
    .unwrap();
            //create token and session
            let token = create_token();
            let _ = create_session(&req.username, &token, &collections.sessions_collection)
                .await
                .unwrap();
            //println!("{:?}", new_user);
            let response = response_models::SignupModel {
                success: success,
                message: message,
            };
            return builder
                .header(
                    "Set-Cookie",
                    format!("token={}; SameSite=lax; path=/", token), // todo: cookie life
                )
                .body(serde_json::to_string(&response).unwrap());
        }
        Err(error) => {
            println!("{:?}", error);
            message = String::from("error signing up");
            let response = response_models::SignupModel {
                success: success,
                message: message,
            };
            return builder.body(serde_json::to_string(&response).unwrap());
        }
    }
}

#[handler]
async fn signin(req: &Request, auth: Data<&fireauth::FireAuth>) {}

#[handler]
async fn signout(req: &Request, auth: Data<&fireauth::FireAuth>) {}

#[handler]
async fn is_signedin(req: &Request, auth: Data<&fireauth::FireAuth>) {}

#[handler]
async fn add_contact(req: &Request, auth: Data<&fireauth::FireAuth>) {}

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
        .sign_in_email(email, password, return_secure_token)
        .await
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
fn ws(
    //Path(name): Path<String>,
    ws: WebSocket,
    req: &Request,
    connection_manager: Data<&Arc<RwLock<ConnectionManager>>>, //sender: Data<&tokio::sync::broadcast::Sender<String>>,
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
        .at("/signup", post(signup))
        .at("/signin", post(signin))
        .at("/signout", post(signout))
        .at("/isSignedin", post(is_signedin))
        .at("/addContact", post(add_contact))
        .at(
            "/ws",
            get(ws), //.data(tokio::sync::broadcast::channel::<String>(32).0)),
        )
        .at("/hello", get(hello))
        .with(CookieJarManager::new())
        .data(connection_manager)
        .data(auth)
        .data(collections);

    Server::new(TcpListener::bind("127.0.0.1:5000"))
        .run(app)
        .await
}
