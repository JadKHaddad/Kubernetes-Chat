const fs = require('fs');
const mongoose = require('mongoose');
const config = JSON.parse(fs.readFileSync('../Config/config.json'));
const mongoUri = `mongodb://${config.mongo_host}:${config.mongo_port}/${config.mongo_db_name}`;

mongoose.connect(mongoUri, {useNewUrlParser: true});

const UserSchema = mongoose.Schema({
  localId: {
    type: String
  },
  email: {
    type: String
  },
  username: {
    type: String
  },
  signupTimeStamp: {
    type: Number
  },
  contacts: {
    type: Array
  },
  subscribers: {
    type: Array
  }
}, { collection: 'Users' });
const SessionSchema = mongoose.Schema({
  token: {
    type: String
  },
  username: {
    type: String
  }
}, { collection: 'Sessions' });

const Session = mongoose.model('Session', SessionSchema );
const User = mongoose.model('User', UserSchema );

module.exports = { User, Session};