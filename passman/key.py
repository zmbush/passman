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
        self.key_type = 'invalid'

    def ensure(self, key):
        if key not in self.__dict__:
            self.__dict__[key] = random_str()

    def ensureAll(self):
        self.ensure('alias')
        self.ensure('url')
        self.ensure('username')
        self.ensure('password')
        self.ensure('notes')

    def setSite(self, url, username, password, alias=None, notes=""):
        if alias is None:
            alias = url

        self.key_type = 'site'
        self.alias = alias
        self.url = url
        self.username = username
        self.password = password
        self.notes = notes

    def encode(self):
        self.ensureAll()
        return json.dumps(self.__dict__)

    def decode(self, string):
        try:
            self.__dict__ = json.loads(string)
        except:
            self.key_type = 'invalid'
