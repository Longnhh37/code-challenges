# ccc06s2
import string

alphabet = set(string.ascii_uppercase + ' ')

plain = input()
cipher = input()
msg = input()

d = {}
for i in range(len(plain)):
    d[plain[i]] = cipher[i]

if len(d) == 26:
    missing_plain = alphabet - set(d.keys())
    missing_cipher = alphabet - set(d.values())
    d[missing_plain.pop()] = missing_cipher.pop()
        
reversed_d = {v: k for k, v in d.items()}

print(''.join(reversed_d.get(ch, '.') for ch in msg))

