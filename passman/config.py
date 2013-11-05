from Crypto import Random
import json
import base64


class Config(object):
    def __init__(self):
        pass

    def encode(self):
        self.salt = base64.b64encode(Random.new().read(256))
        return json.dumps(self.__dict__)

    def decode(self, string):
        try:
            self.__dict__ = json.loads(string)
        except:
            self.salt = base64.b64encode(Random.new().read(256))

    def __getattribute__(self, name):
        if name == "salt":
            return base64.b64decode(self.__dict__['salt'])
        else:
            return object.__getattribute__(self, name)
