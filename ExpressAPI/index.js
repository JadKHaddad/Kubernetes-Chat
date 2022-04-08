const express = require('express');
const enableWs = require('express-ws');
const compression = require('compression');
const helmet = require('helmet');
const fs = require('fs');
const { User, Session } = require("./mods/mongo.js");
const { auth, createUserWithEmailAndPassword, signInWithEmailAndPassword } = require("./mods/firebase.js");

const config = JSON.parse(fs.readFileSync('../Config/config.json'));
const ConnectionManager = require("./mods/connectionManager.js");

const app = express()
enableWs(app)
const port = 5000
const connectionManager = new ConnectionManager(config.redis_host, config.redis_port);
app.use(express.json())
app.use(compression());
app.use(helmet());

//TESTS
app.get('/test', async (req, res) => {
  const response = JSON.stringify({ message: await connectionManager.testSetGet() });
  res.setHeader('Content-Type', 'application/json');
  res.send(response);
})

app.get('/', async (req, res) => {
  let users = []
  const users_ = await User.find();
  for (const [key, value] of Object.entries(users_)) {
    users.push(value.username)
  }
  const response = JSON.stringify({ success: true, users })
  res.setHeader('Content-Type', 'application/json');
  res.send(response);
})
//ENDOFTESTS

function getCookies(request) {
  var cookies = {};
  if (request.headers.cookie === undefined) return cookies
  request.headers && request.headers.cookie.split(';').forEach(function (cookie) {
    var parts = cookie.match(/(.*?)=(.*)$/)
    cookies[parts[1].trim()] = (parts[2] || '').trim();
  });
  return cookies;
};


function createToken() {
  let result = '';
  let characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let charactersLength = characters.length;
  for (var i = 0; i < 16; i++) {
    result += characters.charAt(Math.floor(Math.random() *
      charactersLength));
  }
  return result;
}


async function createSession(username, token) {
  const session = new Session({
    token, username
  });
  await session.save();
}


async function validateToken(token) {
  let username = null;
  let message = "no token";
  if (token !== undefined) {
    const user = await Session.findOne({ "token": token });
    if (user) {
      username = user.username;
      message = "success";
    } else {
      message = "token is no valid";
    }
  }
  return { username, message }
}


async function getUserInfo(username) {
  let info = {};
  const user = User.findOne({ username });
  if (user != null) {
    info.contacts = user.contacts;
  }
  return info;
}


app.post('/signup', async (req, res) => {
  let success = false
  let token = null
  let message = "username exists"
  let email = req.body.email
  let username = req.body.username
  let password = req.body.password
  // check data base if username does not exist
  const oldUser = await User.findOne({ email: email })
  if (!oldUser) {
    try {
      let user = await createUserWithEmailAndPassword(auth, email, password);
      // create user and save in data base
      const newUser = new User({
        localId: user.localId, // this is not really working
        email,
        username,
        signupTimeStamp: Date.now() / 1000,
        contacts: [],
        subscribers: []
      });
      await newUser.save();
      // create token and insert into sessions
      token = createToken();
      await createSession(username, token);
      success = true
      message = "success"
    } catch (error) {
      console.log(error.message)
      message = "error signing up"
    }
  }
  const response = JSON.stringify({ success, message })
  if (success) res.cookie('token', token, { maxAge: 24 * 60 * 60 * 1000 * 30 });
  res.setHeader('Content-Type', 'application/json');
  res.send(response);
})


app.post('/signin', async (req, res) => {
  let success = false
  let token = null
  let message = "username exists"
  let username = null
  let email = req.body.email
  let password = req.body.password
  try {
    await signInWithEmailAndPassword(auth, email, password);
    const user = await User.findOne({ email: email })
    if (user) {
      username = user.username;
      // create token and insert into sessions
      token = createToken();
      await createSession(username, token);
      success = true
      message = "success"
    } else {
      message = "error signing in"
    }
  } catch (error) {
    console.log(error.message)
    message = "error signing in"
  }
  const response = JSON.stringify({ success, username, info: await getUserInfo(username), message })
  if (success) res.cookie('token', token, { maxAge: 24 * 60 * 60 * 1000 * 30 });
  res.setHeader('Content-Type', 'application/json');
  res.send(response);
})


/*
app.post('/signout', async (req, res) => {
  // delete token as session
})
*/


app.post('/isSignedin', async (req, res) => {
  const token = getCookies(req).token
  const { username, message } = await validateToken(token)
  res.setHeader('Content-Type', 'application/json');
  res.send(JSON.stringify({ success: username != null, username: username, message: message }));
})


app.post('/addContact', async (req, res) => {
  let success = false;
  const token = getCookies(req).token;
  let { username, message } = await validateToken(token);
  if (username != null) {
    const usernameTo = req.body.username;
    const user = await User.findOne({ username: usernameTo });
    if (user != null) {
      await User.updateOne({ username: username }, { "$addToSet": { contacts: usernameTo } })
      await User.updateOne({ username: usernameTo }, { "$addToSet": { subscribers: username } })
      success = true;
    } else {
      message = "username does not exist"
    }
  }
  res.setHeader('Content-Type', 'application/json');
  res.send(JSON.stringify({ success: success, message: message }));
})


app.ws('/ws', async (ws, req) => {
  const token = getCookies(req).token
  const { username, message } = await validateToken(token);
  if (username == null) {
    return
  }
  let subscribers = []
  const user = await User.findOne({ username: username })

  if (user != null) subscribers = user.subscribers

  connectionManager.connect(username, ws, subscribers);
  ws.on('message', async (msg) => {
    msg = JSON.parse(msg)
    const messageType = msg.type;
    const usernameTo = msg.username_to;
    if (messageType === "typing") {
      await connectionManager.sendPersonalTyping(username, usernameTo);
      return
    } else if (messageType === "txt") {
      const textContent = msg.text_content;
      await connectionManager.sendPersonalMessage(username, usernameTo, textContent);
      return
    } else if (messageType === "like") {
      const messageId = msg.message_id;
      await connectionManager.sendPersonalLike(username, usernameTo, messageId);
      return
    } else if (messageType === "status_request") {
      const usernames = msg.usernames;
      await connectionManager.requestStatus(username, usernames);
      return
    }
  })

  ws.on('close', () => {
    connectionManager.disconnect(username, ws);
  })
})

app.listen(port, () => {
  console.log(`API listening on port ${port}`)
})