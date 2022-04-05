const Redis = require('redis');


function removeFromArray(array, value) {
  return array.filter((element) => {
    return element != value;
  });
}


class ConnectionManager {
  constructor(redisHost, redisPort) {
    this.id = (Date.now() / 1000).toString();
    this.sessions = {};
    this.red = Redis.createClient({
      url: `redis://${redisHost}:${redisPort}`
    });
    this.sub = this.red.duplicate();
    this.red.connect();
    this.sub.connect();
    this.sub.subscribe('events', (message) => {
      this.listenToPublisher(message);
    });
  }

  //TESTS
  async testSetGet() {
    await this.red.set('test', 'test');
    let test = await this.red.get('test');
    return test;
  }
  //ENDOFTESTS

  listenToPublisher(message) {
    const data = JSON.parse(message);
    const command = data.command;
    if (command != null) {
      const commandString = command.command;
      if (commandString === "add_to:subscribers") {
        const usernameTarget = command.username_target;
        const subscriber = command.subscriber;
        if (this.sessions.hasOwnProperty(usernameTarget)) {
          this.sessions[usernameTarget]["subscribers"].push(subscriber)
        }
      }
    } else {
      const usernameTarget = data.username_target;
      const messageObj = data.message_dict;
      if (this.sessions.hasOwnProperty(usernameTarget)) {
        for (var i = 0; i < this.sessions[usernameTarget]["sockets"].length; i++) {
          this.sessions[usernameTo]["sockets"][i].send(JSON.stringify(messageObj))
        }
      }
    }
  }


  async notify(usernameTarget, messageObj, command) {
    const serverIds = await this.red.sMembers(usernameTarget);
    for (var i = 0; i < serverIds.length; i++) {
      this.red.publish(serverIds[i], JSON.stringify({ username_target: usernameTarget, message_dict: messageObj, command: command }));
    }
  }


  async connect(username, websocket, subscribers) {
    if (!this.sessions.hasOwnProperty(username)) {
      this.sessions[username] = { sockets: [websocket], subscribers: subscribers };
      // add server to user
      await this.red.sAdd(username, this.id);
      // notify subs
      const messageObj = { event_type: "status", username_target: username, status: "online" };
      for (var i = 0; i < subscribers.length; i++) {
        const subscriber = subscribers[i];
        if (this.sessions.hasOwnProperty(subscriber)) {
          for (var j = 0; j < this.sessions[subscriber]["sockets"].length; j++) {
            this.sessions[subscriber]["sockets"][j].send(JSON.stringify(messageObj))
          }
        }
      }
    } else {
      this.sessions[username]["sockets"].push(websocket);
    }
    console.log(this.sessions)
  }


  async disconnect(username, websocket) {
    if (this.sessions.hasOwnProperty(username)) {
      this.sessions[username]["sockets"] = removeFromArray(this.sessions[username]["sockets"], websocket);
      if (this.sessions[username]["sockets"].length < 1) {
        // remove serve from user
        await this.red.sRem(username, this.id);
        // check if user is offline
        const servers = await this.red.sMembers(username);
        if (servers.length < 1) {
          const messageObj = { event_type: "status", username_target: username, status: "offline" };
          const subscribers = this.sessions[username]["subscribers"];
          for (var i = 0; i < subscribers.length; i++) {
            const subscriber = subscribers[i];
            if (this.sessions.hasOwnProperty(subscriber)) {
              for (var j = 0; j < this.sessions[subscriber]["sockets"].length; j++) {
                this.sessions[subscriber]["sockets"][j].send(JSON.stringify(messageObj));
                console.log("offline sent")
              }
            }
            await this.notify(subscriber, messageObj);
          }
        }
        delete this.sessions[username];
      }
    }
  }


  async addToSubscribers(subscriber, usernameTo) {
    if (!this.sessions.hasOwnProperty(usernameTo)) {
      this.sessions[usernameTo]["subscribers"].push(subscriber);
    }
    const command = { command: "add_to_subscribers", subscriber: subscriber, username_target: usernameTo };
    await this.notify(usernameTo, null, command);
  }


  async requestStatus(subscriber, usernamesTo) {
    for (var i = 0; i < usernamesTo.length; i++) {
      const usernameTo = usernamesTo[i];
      const servers = await this.red.sMembers(usernameTo);
      if (servers.length > 0) {
        const messageObj = { event_type: "status", username_target: usernameTo, status: "online" };
        if (this.sessions.hasOwnProperty(subscriber)) {
          for (var j = 0; j < this.sessions[subscriber]["sockets"].length; j++) {
            this.sessions[subscriber]["sockets"][j].send(JSON.stringify(messageObj));
          }
        }
        await this.notify(subscriber, messageObj);
      }
    }
  }

  async sendPersonalTyping(usernameFrom, usernameTo) {
    const messageObj = { event_type: "typing", "username_target": usernameFrom };
    if (this.sessions.hasOwnProperty(usernameTo)) {
      for (var i = 0; i < this.sessions[usernameTo]["sockets"].length; i++) {
        this.sessions[usernameTo]["sockets"][i].send(JSON.stringify(messageObj))
      }
    }
    await this.notify(usernameTo, messageObj);
  }


  async sendPersonalMessage(usernameFrom, usernameTo, textContent) {
    const today = new Date();
    const date = today.getHours() + ":" + today.getMinutes() + ":" + today.getSeconds();
    const messageId = Date.now() / 1000;
    let messageObj = { event_type: "msg", event_content: { username_target: usernameTo, message_content: { type: "txt", text_content: textContent, date: date, id: messageId, received: false } } };
    if (this.sessions.hasOwnProperty(usernameFrom)) {
      for (var i = 0; i < this.sessions[usernameFrom]["sockets"].length; i++) {
        this.sessions[usernameFrom]["sockets"][i].send(JSON.stringify(messageObj))
      }
    }
    await this.notify(usernameFrom, messageObj);
    messageObj = { event_type: "msg", event_content: { username_target: usernameFrom, message_content: { type: "txt", text_content: textContent, date: date, id: messageId, received: true } } }
    if (this.sessions.hasOwnProperty(usernameTo)) {
      for (var i = 0; i < this.sessions[usernameTo]["sockets"].length; i++) {
        this.sessions[usernameTo]["sockets"][i].send(JSON.stringify(messageObj))
      }
    }
    await this.notify(usernameTo, messageObj);
  }


  async sendPersonalLike(usernameFrom, usernameTo, messageId) {
    let messageObj = { event_type: "like", username_target: usernameTo, message_id: messageId, self_like: true };
    if (this.sessions.hasOwnProperty(usernameFrom)) {
      for (var i = 0; i < this.sessions[usernameFrom]["sockets"].length; i++) {
        this.sessions[usernameFrom]["sockets"][i].send(JSON.stringify(messageObj))
      }
    }
    await this.notify(usernameFrom, messageObj);
    messageObj = { event_type: "like", username_target: usernameFrom, message_id: messageId };
    if (this.sessions.hasOwnProperty(usernameTo)) {
      for (var i = 0; i < this.sessions[usernameTo]["sockets"].length; i++) {
        this.sessions[usernameTo]["sockets"][i].send(JSON.stringify(messageObj))
      }
    }
    await this.notify(usernameTo, messageObj);
  }

}

module.exports = ConnectionManager;