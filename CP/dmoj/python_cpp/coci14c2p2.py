#coci14c2p2
from collections import Counter
import sys

input()
s = sys.stdin.read().split()

c = Counter(s)
print(next(k for k, v in c.items() if v % 2))

