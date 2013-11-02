import base64
import os
import json


def random_str(b=32):
    return base64.b64encode(os.urandom(b))


class Key(object):
    '''
    There is currently only one type of key. And that is 'Site':

    Site = {'type': 'site',
            'alias': '...',
            'url': '...',
            'username': '...',
            'password': '...',
            'notes': '...'}

    '''

    def __init__(self):
        self.data = {
            'type': 'invalid'
        }
        self.ensureAll()

    def ensure(self, key):
        if key not in self.data:
            self.data[key] = random_str()

    def ensureAll(self):
        self.ensure('alias')
        self.ensure('url')
        self.ensure('username')
        self.ensure('password')
        self.ensure('notes')

    def setSite(self, url, username, password, alias=None, notes=""):
        if alias is None:
            alias = url

        self.data['type'] = 'site'
        self.data['alias'] = alias
        self.data['url'] = url
        self.data['username'] = username
        self.data['password'] = password
        self.data['notes'] = notes

    def encode(self):
        return json.dumps(self.data)

    def decode(self, string):
        try:
            self.data = json.loads(string)
        except:
            self.data = {'type': 'invalid'}
        finally:
            self.ensureAll()
