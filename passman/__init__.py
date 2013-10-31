from Crypto.Cipher import AES
from Crypto.Hash import SHA256
import base64

BLOCK_SIZE = 32
PADDING = "#"

if __name__ == "__main__":
    password = SHA256.new('this is a password and it is rather long.').digest()

    pad = lambda s: s + (BLOCK_SIZE - len(s) % BLOCK_SIZE) * PADDING

    c = AES.new(password)

    enc = base64.b64encode(c.encrypt(pad("hey")))
    print(enc)
    dec = c.decrypt(base64.b64decode(enc)).rstrip(PADDING)
    print(dec)
