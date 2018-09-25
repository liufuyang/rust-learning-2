
```sh
FLASK_APP=server.py flask run -h localhost

ab -n 200 -c 50 127.0.0.1:5000/add

ab -n 200 -c 50 localhost:8000/add
```


