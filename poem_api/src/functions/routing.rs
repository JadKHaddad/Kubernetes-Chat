use futures_util::{SinkExt, StreamExt};
use mongodb::bson::doc;
use parking_lot::RwLock;
use poem::{
    handler,
    http::{header, StatusCode},
    web::{
        websocket::{CloseCode, Message, WebSocket},
        Data, Json, Path,
    },
    IntoResponse, Request, Response,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::connection_manager::ConnectionManager;
use crate::functions::statics;
use crate::models::*;

#[handler]
pub async fn signup(
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
            let token = statics::create_token();
            let _ =
                statics::create_session(&req.username, &token, &collections.sessions_collection)
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
pub async fn signin(
    req: Json<request_models::SigninModel>,
    auth: Data<&fireauth::FireAuth>,
    collections: Data<&mongodb_models::Collections>,
) -> Response {
    let mut builder = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK);

    let mut response = response_models::SigninModel {
        success: false,
        username: String::new(),
        info: UserInfo { contacts: None },
        message: String::new(),
    };

    match auth.sign_in_email(&req.email, &req.password, true).await {
        Ok(_) => (),
        Err(error) => {
            println!("{:?}", error);
            response.message = String::from("error signing in");
            return builder.body(serde_json::to_string(&response).unwrap());
        }
    }
    response.success = true;
    let res = collections
        .users_collection
        .find_one(
            doc! {
                "email": &req.email
            },
            None,
        )
        .await
        .unwrap()
        .unwrap();
    response.username = res.get_str("username").unwrap().to_string();
    response.info.contacts = Some(res.get_array("contacts").unwrap());
    //create token and session
    let token = statics::create_token();
    let _ = statics::create_session(&response.username, &token, &collections.sessions_collection)
        .await
        .unwrap();
    //statics::get_user_info(&req.email, &collections.users_collection).await;
    return builder.body(serde_json::to_string(&response).unwrap());
}

#[handler]
pub async fn signout(req: &Request, auth: Data<&fireauth::FireAuth>) {}

#[handler]
pub async fn is_signedin(
    req: &Request,
    collections: Data<&mongodb_models::Collections>,
) -> Response {
    let mut builder = Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK);
    let mut response = response_models::IsSignedinModel {
        success: false,
        username: None,
        message: String::from("no token"),
    };

    if let Some(token_value) = req.cookie().get("token") {
        match statics::validate_token(token_value.value_str(), &collections.sessions_collection)
            .await
        {
            Ok((username, message)) => {
                response.success = true;
                response.username = username;
                response.message = message;
            }
            Err(error) => {
                // connection error
                println!("{:?}", error);
            }
        };
    }
    return builder.body(serde_json::to_string(&response).unwrap());
}

#[handler]
pub async fn add_contact(req: &Request, auth: Data<&fireauth::FireAuth>) {}

#[handler]
pub async fn hello(
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

struct Hello(Option<String>);

impl IntoResponse for Hello {
    fn into_response(self) -> Response {
        let msg = match self.0 {
            Some(name) => format!("hello {}", name),
            None => format!("hello"),
        };
        msg.into_response()
    }
}

#[handler]
pub fn ws(
    ws: WebSocket,
    req: &Request,
    connection_manager: Data<&Arc<RwLock<ConnectionManager>>>, //sender: Data<&tokio::sync::broadcast::Sender<String>>,
) -> impl IntoResponse {
    //validate token
    let username = String::from("hahaha");
    if let None = req.cookie().get("token") {}
    // cant get ws to disconnect




    let (tx, mut rx1) = tokio::sync::watch::channel(String::from("hello"));
    let con = Arc::clone(&connection_manager);

    //alot of issues here
    // find out how to save the name
    // check the disconnections if sender or reciever is closed
    return ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();

        let sender_username = String::from(&username);
        let sender_username_2 = String::from(&sender_username);
        let socket_posistion: usize;
        {
            let mut con = con.write();
            socket_posistion = con.connect(username, tx);
        }

        tokio::spawn(async move {
            while let Some(Ok(msg)) = stream.next().await {
                if let Message::Text(rec) = msg {
                    println!("from: {}, {}", sender_username, rec);
                    //let con = con.read();
                    //con.send_personal_message(&sender_username, &to, String::from("sup sup"));

                    //     break;
                    // }
                    //s.send(Message::Text(format!("sent to self, {}", text))).await.unwrap();

                    //     .await
                    // if sink
                    //     .send(Message::Text(format!("sent to self, {}", text)))
                    //     .await
                    //     .is_err()
                    // {
                    //     break;
                    // }
                    // if sender.send(format!("{}: {}", name, text)).is_err() {
                    //     break;
                    // }
                }
            }
            let mut con = con.write();
            con.disconnect(sender_username, socket_posistion);
            //con.sessions.remove(&sender_name);
            //println!("{}: sender disconnected", sender_name);
        });

        tokio::spawn(async move {
            while rx1.changed().await.is_ok() {
                //println!("received = {:?}", *rx1.borrow());
                let msg = String::from(&*rx1.borrow());
                if sink.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
            /*while let Some(msg) = rx1.recv().await {
                println!("sent: {}", msg);
                if sink.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }*/
            println!("ONE RES DISCONNECTED: {}", sender_username_2);
        });
    });
}
