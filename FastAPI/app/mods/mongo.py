import json
from typing import Final
from pymongo import MongoClient

config = {}
with open ('../Config/config.json', "r", encoding="utf-8") as file:
    config= json.load(file)

MONGODB_HOST: Final = config['mongo_host']
MONGODB_PORT: Final = config['mongo_port']
DB_NAME: Final = config['mongo_db_name']

CONNECTION: Final = MongoClient(MONGODB_HOST, MONGODB_PORT, maxPoolSize=200)
USERS_COLLECTION: Final = CONNECTION[DB_NAME]["Users"]
SESSIONS_COLLECTION: Final = CONNECTION[DB_NAME]["Sessions"]