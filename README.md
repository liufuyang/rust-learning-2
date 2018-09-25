
```sh
FLASK_APP=server.py flask run

ab -n 200 -c 50 127.0.0.1:5000/add
```


