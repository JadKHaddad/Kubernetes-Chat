from locust import HttpUser, task, between

class User(HttpUser):
    wait_time = between(1, 5)

    @task
    def index(self):
        self.client.get("/test_mongo")