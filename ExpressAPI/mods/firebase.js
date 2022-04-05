const fs = require('fs');
const { initializeApp } = require("firebase/app");
const { getAuth, createUserWithEmailAndPassword, signInWithEmailAndPassword } = require("firebase/auth");
const firebaseConfig = JSON.parse(fs.readFileSync('../FirebaseConfig/config.json'));
const firebase = initializeApp(firebaseConfig);
const auth = getAuth();

module.exports = { auth, createUserWithEmailAndPassword, signInWithEmailAndPassword };