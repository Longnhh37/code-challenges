# dmopc19c3p1
from collections import Counter

n = int(input())
data = list(map(int, input().split()))

d = Counter(data)
mode = max(d.values())

out = sorted(k for k, v in d.items() if v == mode)
print(*out, sep=' ')