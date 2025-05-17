from pyaes import AES, CTR
from secrets import token_bytes

key = token_bytes(16)
nonce = token_bytes(8)
aes = AES.init(CTR(), nonce, key)
pt = b"Hello World from PyAES!"
ct = aes.encrypt(pt)
print(f"Ciphertext: {ct.hex()}")
print(f"Decrypted: {aes.decrypt(ct)}")


