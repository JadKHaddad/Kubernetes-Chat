use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use parking_lot::RwLock;
use poem::{
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    middleware::{CookieJarManager, AddData},
    session::{CookieConfig, ServerSession, MemoryStorage, Session},
    web::{
        websocket::{Message, WebSocket},
        Data, Html, Path,
    },
    EndpointExt, IntoResponse, Request, Response, Result, Route, Server,
};

#[derive(Clone, Debug)]
struct st{
    count: i32,
}

impl st {
    pub fn inc(&mut self){
        self.count += 1;
    }
}

#[handler]
async fn hello(req: &Request, /*, connection_manager: Data<&ConnectionManager>*/s: Data<&Arc<RwLock<st>>>) -> Response {
    let email = "something@email.com";
    let password = "supersecretji";
    let return_secure_token = true;


    let mut w = s.write();
    w.count = w.count + 1;
    println!("{}", w.count);

    //println!("connection manager: {}", connection_manager.name());
    // match auth
    //     .sign_in_email(email, password, return_secure_token)
    //     .await
    // {
    //     Ok(response) => println!("{:?}", response),
    //     Err(error) => println!("{:?}", error),
    // }
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
    let auth = fireauth::FireAuth::new(String::from("APIKEY"));


    let s = Arc::new(RwLock::new(st{count: 0}));

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/", get(index))
        .at(
            "/ws",
            get(ws), //.data(tokio::sync::broadcast::channel::<String>(32).0)),
        )
        .at("/hello", get(hello))
        .with(CookieJarManager::new()).data(s);


    Server::new(TcpListener::bind("127.0.0.1:5000"))
        .run(app)
        .await
}
