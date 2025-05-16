import pyaes

pt = b"a"*16
print(pt.hex())
k = b"x"*16
ct = pyaes.encrypt(k,pt)
print(ct.hex())
recovered = pyaes.decrypt(k,ct)
print(recovered.hex())

