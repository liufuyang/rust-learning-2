import time
from flask import Flask

app = Flask(__name__)


class SomeObj():
    def __init__(self, param):
        self.param = param

    def query(self):
        return self.param

    def add(self):
        p = self.param
        time.sleep(0.03)
        p += 1
        self.param = p
        return self.param

global_obj = SomeObj(0)


@app.route('/')
def hello_world():
    return 'Hello, World!'


@app.route('/count')
def count():
    return str(global_obj.query())


@app.route('/add')
def add():
    return str(global_obj.add())
