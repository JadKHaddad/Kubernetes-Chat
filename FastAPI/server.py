# run with 
# sudo gunicorn app.main:app --workers 4 --worker-class uvicorn.workers.UvicornWorker --bind 0.0.0.0:5000

import uvicorn

if __name__ == '__main__':
    uvicorn.run("app.main:app",
                host="0.0.0.0",
                port=5000,
                reload=True,
                )