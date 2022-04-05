import json
from typing import Final
from firebase import Firebase

# fire base is just used for authentication
# cred = credentials.Certificate("config/chatfastapi-firebase-adminsdk-etevz-fa72274f85.json")
# firebase_admin.initialize_app(cred)

config = {}
with open ('../FirebaseConfig/config.json', "r", encoding="utf-8") as file:
    config= json.load(file)

firebase = Firebase(config)

AUTH: Final = firebase.auth()