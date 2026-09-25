#ccc11s1
import sys

text = sys.stdin.read()

s = text.lower().count('s')
t = text.lower().count('t')

if t > s:
    print("English")
else:
    print("French")


