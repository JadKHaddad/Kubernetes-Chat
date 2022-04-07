import json
import string
import random
import time
from typing import Optional, Final
from fastapi import FastAPI, Cookie, WebSocket, WebSocketDisconnect, Request
from fastapi.responses import JSONResponse
from fastapi.middleware.cors import CORSMiddleware
from requests.exceptions import HTTPError
from app.mods.connectionManager import ConnectionManager
from app.mods.firebase import AUTH
from app.mods.mongo import USERS_COLLECTION, SESSIONS_COLLECTION

config = {}
with open("../Config/config.json", "r", encoding="utf-8") as file:
    config = json.load(file)

app = FastAPI()


def create_token():
    return str(
        "".join(
            random.choices(
                string.ascii_uppercase + string.ascii_lowercase + string.digits, k=16
            )
        )
    )


def create_session(username, token):
    SESSIONS_COLLECTION.insert_one({"token": token, "username": username})


def validate_token(token):
    username = None
    message = "no token"
    if token is not None:
        user = SESSIONS_COLLECTION.find_one({"token": token})
        if user is not None:
            username = user["username"]
            message = "success"
        else:
            message = "token is no valid"
    return username, message


def get_user_info(username):
    info = {}
    user = USERS_COLLECTION.find_one({"username": username})
    if user is not None:
        info["contacts"] = user["contacts"]
    return info


# origins = [
#     "http://localhost:8080",
# ]

# app.add_middleware(
#     CORSMiddleware,
#     allow_origin_regex= f'http:\/\/{constants.IP}:.*',
#     allow_origins=origins,
#     allow_credentials=True,
#     allow_methods=['*'],
#     allow_headers=['*'],
# )

manager: Final = ConnectionManager(config["redis_host"], config["redis_port"])


# TESTS #


@app.get("/test_redis", response_class=JSONResponse)
async def test_redis():
    return {"message": manager.test_set_get()}


@app.get("/test_mongo", response_class=JSONResponse)
async def test_mongo():
    new_user = {
        "localId": "qwasdyxc",
        "email": "email",
        "username": "username",
        "signupTimeStamp": "adyxaeqwe",
        "contacts": [],
        "subscribers": [],
    }
    USERS_COLLECTION.insert_one(new_user)
    user = USERS_COLLECTION.find_one({"username": "username"})
    mes = user.get("username")
    return {"message": mes}


@app.get("/", response_class=JSONResponse)
async def index():
    # catch mongo connection refused
    docs = USERS_COLLECTION.find()
    users = []
    for doc in docs:
        users.append(doc["username"])
    response = JSONResponse(content={"success": True, "users": users})
    return response


@app.get("/nomongo", response_class=JSONResponse)
async def nomongo():
    response = JSONResponse(content={"success": True})
    return response


# ENDOFTESTTS #


@app.post("/signup", response_class=JSONResponse)
async def signup(request: Request):
    success = False
    token = None
    message = "username exists"
    request_body = await request.json()
    # todo: basemodel
    email = request_body.get("email")
    username = request_body.get("username")
    password = request_body.get("password")
    if USERS_COLLECTION.find_one({"username": username}) is None:  # username available
        try:
            user = AUTH.create_user_with_email_and_password(email, password)
            # create user and save in data base
            new_user = {
                "localId": user["localId"],
                "email": email,
                "username": username,
                "signupTimeStamp": time.time(),
                "contacts": [],
                "subscribers": [],
            }
            USERS_COLLECTION.insert_one(new_user)
            token = create_token()
            create_session(username, token)
            success = True
            message = "success"
        except HTTPError as http_error:
            print(http_error.strerror)
            message = "error signing up"

    response = JSONResponse(content={"success": success, "message": message})
    if success:
        response.set_cookie(key="token", value=token, expires=90000 * 30)
    return response


@app.post("/signin", response_class=JSONResponse)  # sets a cookie (token)
async def signin(request: Request):
    request_body = await request.json()
    success = False
    token = None
    username = None
    message = ""
    email = request_body.get("email")
    password = request_body.get("password")
    try:
        AUTH.sign_in_with_email_and_password(email, password)
        user = USERS_COLLECTION.find_one({"email": email})
        if user is not None:
            username = user["username"]
            token = create_token()
            create_session(username, token)
            success = True
        else:
            message = "error signing in"
    except HTTPError as http_error:
        print(http_error.strerror)
        message = "error signing in"

    response = JSONResponse(
        content={
            "success": success,
            "username": username,
            "info": get_user_info(username),
            "message": message,
        }
    )
    if success:
        response.set_cookie(key="token", value=token, expires=90000 * 30)
    return response


@app.post("/signout", response_class=JSONResponse)  # removes a cookie (token)
async def signout():
    response = JSONResponse(content={"success": True})
    response.delete_cookie("token")
    return response


@app.post("/isSignedin", response_class=JSONResponse)  # checks a cookie (token)
async def is_signedin(token: Optional[str] = Cookie(None)):
    username, message = validate_token(token)
    response = JSONResponse(
        content={
            "success": username is not None,
            "username": username,
            "message": message,
        }
    )
    return response


@app.post("/addContact", response_class=JSONResponse)
async def add_contact(request: Request, token: Optional[str] = Cookie(None)):
    success = False
    message = ""
    username_from, message = validate_token(token)
    if username_from is not None:
        request_body = await request.json()
        username_to = request_body.get("username")
        # ceck if username exists
        user = USERS_COLLECTION.find_one({"username": username_to})
        if user is not None:
            # add user to contacts
            USERS_COLLECTION.update_one(
                {"username": username_from}, {"$addToSet": {"contacts": username_to}}
            )
            # add username_from to subscribers of username_to
            USERS_COLLECTION.update_one(
                {"username": username_to}, {"$addToSet": {"subscribers": username_from}}
            )
            manager.add_to_subscribers(username_from, username_to)
            success = True
        else:
            message = "username does not exist"
    response = JSONResponse(content={"success": success, "message": message})
    return response


@app.websocket("/ws")  # checks a cookie (token)
async def websocket_endpoint(websocket: WebSocket, token: Optional[str] = Cookie(None)):
    username_from, message = validate_token(token)
    if username_from is None:
        return
    # get subs
    subscribers = []
    user = USERS_COLLECTION.find_one({"username": username_from})
    if user is not None:
        subscribers = user["subscribers"]
    await manager.connect(username_from, websocket, set(subscribers))
    try:
        while True:
            data = await websocket.receive_text()
            data = json.loads(data)
            message_type = data.get("type")
            username_to = data.get("username_to")

            if message_type == "typing":
                await manager.send_personal_typing(username_from, username_to)
            elif message_type == "txt":
                text_content = data.get("text_content")
                await manager.send_personal_message(
                    username_from, username_to, text_content
                )
            elif message_type == "like":
                message_id = data.get("message_id")
                await manager.send_personal_like(username_from, username_to, message_id)
            elif message_type == "status_request":
                usernames = data.get("usernames")
                await manager.request_status(username_from, usernames)
            # await manager.broadcast(data)

    except WebSocketDisconnect:

        await manager.disconnect(username_from, websocket)
