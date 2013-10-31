from Crypto.Cipher import AES
import os
import base64

BLOCK_SIZE = 32

secret = os.urandom(BLOCK_SIZE)
cipher = AES.new(secret)

PADDING = "}"

pad = lambda s: s + (BLOCK_SIZE - len(s) % BLOCK_SIZE) * PADDING

enc = base64.b64encode(cipher.encrypt(
                       pad("Hello, my name is zach... This is text")))

print(enc)

dec = cipher.decrypt(base64.b64decode(enc)).rstrip(PADDING)

print dec
