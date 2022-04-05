import json
from typing import List, Dict, Set
from fastapi import WebSocket
import json
from datetime import datetime
import time
import redis
import threading
import asyncio


class ConnectionManager:
    __slots__ = "id", "sessions", "red", "sub", "thread"

    # TEST #

    def test_set_get(self):
        self.red.set("test", "test")
        test = self.red.get("test")
        return test

    # ENDOFTESTTS #

    def __init__(self, redis_host, redis_port):
        self.id = str(time.time())
        self.sessions: Dict = {}
        # check redis connection refused and catch it later
        self.red = redis.Redis(
            connection_pool=redis.ConnectionPool(
                host=redis_host, port=redis_port, db=0, max_connections=200
            ),
            charset="utf-8",
            decode_responses=True,
        )
        self.sub = self.red.pubsub()
        self.sub.subscribe([self.id])
        self.thread = threading.Thread(target=self.between_callback)
        self.thread.daemon = True
        self.thread.start()

    def between_callback(self):
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        loop.run_until_complete(self.listen_to_publisher())
        loop.close()

    async def listen_to_publisher(self):
        for item in self.sub.listen():
            if item["data"] != 1:
                data = json.loads(item["data"])  # needs exception
                command = data["command"]
                if command is not None:
                    command_string = command["command"]
                    if command_string == "add_to_subscribers":
                        username_target = command["username_target"]
                        subscriber = command["subscriber"]
                        if username_target in self.sessions:
                            self.sessions[username_target]["subscribers"].add(
                                subscriber
                            )
                else:
                    username_target = data["username_target"]
                    message_dict = data["message_dict"]
                    if username_target in self.sessions:
                        for socket in self.sessions[username_target]["sockets"]:
                            await socket.send_text(json.dumps(message_dict))

    def notify(self, username_target, message_dict, command=None):
        server_ids = self.red.smembers(username_target)
        for server_id in server_ids:
            if server_id.decode("utf-8") != self.id:
                self.red.publish(
                    server_id,
                    json.dumps(
                        {
                            "username_target": username_target,
                            "message_dict": message_dict,
                            "command": command,
                        }
                    ),
                )

    async def connect(self, username: str, websocket: WebSocket, subscribers: Set[str]):
        await websocket.accept()
        if username not in self.sessions:
            self.sessions[username] = {
                "sockets": [websocket],
                "subscribers": subscribers,
            }
            # add server to user
            self.red.sadd(username, self.id)
            # notify subscribers
            message_dict = {
                "event_type": "status",
                "username_target": username,
                "status": "online",
            }
            for subscriber in subscribers:
                if subscriber in self.sessions:
                    for socket in self.sessions[subscriber]["sockets"]:
                        await socket.send_text(json.dumps(message_dict))
                self.notify(subscriber, message_dict)
        else:
            self.sessions[username]["sockets"].append(websocket)
        print(self.sessions)

    async def disconnect(self, username: str, websocket: WebSocket):
        if username in self.sessions:
            self.sessions[username]["sockets"].remove(websocket)
            if len(self.sessions[username]["sockets"]) < 1:
                # remove server from user
                self.red.srem(username, self.id)
                # check if user is offline
                servers = self.red.smembers(username)
                if len(servers) < 1:
                    message_dict = {
                        "event_type": "status",
                        "username_target": username,
                        "status": "offline",
                    }
                    for subscriber in self.sessions[username]["subscribers"]:
                        if subscriber in self.sessions:
                            for socket in self.sessions[subscriber]["sockets"]:
                                await socket.send_text(json.dumps(message_dict))
                        self.notify(subscriber, message_dict)
                del self.sessions[username]

    def add_to_subscribers(self, subscriber, username_to):
        if username_to in self.sessions:
            self.sessions[username_to]["subscribers"].add(subscriber)
        command = {
            "command": "add_to_subscribers",
            "subscriber": subscriber,
            "username_target": username_to,
        }
        self.notify(username_to, None, command)

    async def request_status(self, subscriber, usernames_to):
        for username_to in usernames_to:
            serves = self.red.smembers(username_to)
            if len(serves) > 0:
                message_dict = {
                    "event_type": "status",
                    "username_target": username_to,
                    "status": "online",
                }
                if subscriber in self.sessions:
                    for socket in self.sessions[subscriber]["sockets"]:
                        await socket.send_text(json.dumps(message_dict))
                self.notify(subscriber, message_dict)

    async def send_personal_typing(self, username_from: str, username_to: str):
        message_dict = {"event_type": "typing", "username_target": username_from}
        if username_to in self.sessions:
            for socket in self.sessions[username_to]["sockets"]:
                await socket.send_text(json.dumps(message_dict))
        self.notify(username_to, message_dict)

    async def send_personal_message(
        self, username_from: str, username_to: str, text_content: str
    ):
        date = datetime.now().strftime("%H:%M:%S")
        message_id = time.time()
        message_dict = {
            "event_type": "msg",
            "event_content": {
                "username_target": username_to,
                "message_content": {
                    "type": "txt",
                    "text_content": text_content,
                    "date": date,
                    "id": message_id,
                    "received": False,
                },
            },
        }
        if username_from in self.sessions:
            for socket in self.sessions[username_from]["sockets"]:
                await socket.send_text(json.dumps(message_dict))
        self.notify(username_from, message_dict)
        message_dict = {
            "event_type": "msg",
            "event_content": {
                "username_target": username_from,
                "message_content": {
                    "type": "txt",
                    "text_content": text_content,
                    "date": date,
                    "id": message_id,
                    "received": True,
                },
            },
        }
        if username_to in self.sessions:
            for socket in self.sessions[username_to]["sockets"]:
                await socket.send_text(json.dumps(message_dict))
        self.notify(username_to, message_dict)

    async def send_personal_like(
        self, username_from: str, username_to: str, message_id: str
    ):
        message_dict = {
            "event_type": "like",
            "username_target": username_to,
            "message_id": message_id,
            "self_like": True,
        }
        if username_from in self.sessions:
            for socket in self.sessions[username_from]["sockets"]:
                await socket.send_text(json.dumps(message_dict))
        self.notify(username_from, message_dict)
        message_dict = {
            "event_type": "like",
            "username_target": username_from,
            "message_id": message_id,
        }
        if username_to in self.sessions:
            for socket in self.sessions[username_to]["sockets"]:
                await socket.send_text(json.dumps(message_dict))
        self.notify(username_to, message_dict)
