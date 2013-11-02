from Crypto.Cipher import AES
from Crypto.Hash import SHA256
import os
import shutil
import key
import base64


def pad(s):
    return s + (32 - len(s) % 32) * "\0"


def unpad(s):
    return s.rstrip("\0")


class Keystore(object):
    def __init__(self, directory, password, max_keys=10000):
        self.directory = directory
        self.max_keys = max_keys
        self.keys = []
        self.passhash = SHA256.new(password).digest()
        self.c = AES.new(self.passhash)

        if not self.exists():
            self.create()
            self.write()
        else:
            self.read()

    def exists(self):
        return (os.path.exists(self.directory) and
                os.path.isdir(self.directory) and
                os.path.exists(os.path.join(self.directory, ".passman")))

    def create(self):
        print "Creating!"
        if self.exists():
            shutil.rmtree(self.directory)
        os.makedirs(self.directory)

        for i in range(self.max_keys):
            self.keys.append(key.Key())

    def write(self):
        print "Writing!"
        open(os.path.join(self.directory, ".passman"), 'a').close()
        for i, k in enumerate(self.keys):
            fname = os.path.join(self.directory, "%06d.pm" % i)
            f = open(fname, 'w+')

            f.write(base64.b64encode(self.c.encrypt(pad(k.encode()))))

            f.close()

    def read(self):
        print "Reading!"
        if not self.exists():
            return

        self.keys = []

        for i in range(self.max_keys):
            fname = os.path.join(self.directory, "%06d.pm" % i)
            f = open(fname, 'r')
            k = key.Key()

            k.decode(unpad(self.c.decrypt(base64.b64decode(f.read()))))

            self.keys.append(k)
