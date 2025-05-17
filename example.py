from pyaes import AES, CTR

key = b"a"*16
nonce = b"x"*8
aes = AES.init(CTR(), nonce, key)
pt = b"Hello World from PyAES!"
ct = aes.encrypt(pt)
print(f"Ciphertext: {ct.hex()}")
print(f"Decrypted: {aes.decrypt(ct)}")


