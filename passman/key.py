import base64
import os
import json


class Key(object):
    def __init__(self, name=None, url=None, password=None):
        self.name = name
        self.url = url
        self.password = password
        self.valid = True

        if password == name == url is None:
            self.name = base64.b64encode(os.urandom(32))
            self.url = base64.b64encode(os.urandom(32))
            self.password = base64.b64encode(os.urandom(32))
            self.valid = False

    def encode(self):
        data = {
            'type': 'base',
            'name': self.name,
            'url': self.url,
            'password': self.password,
            'valid': self.valid
        }
        return json.dumps(data)

    def decode(self, string):
        data = json.loads(string)
        if data['type'] == 'base':
            self.name = data['name']
            self.url = data['url']
            self.password = data['password']
            self.valid = data['valid']
