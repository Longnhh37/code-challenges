# coci17c2p2
from collections import defaultdict

K, N = map(int, input().split())

mp = defaultdict(list)
for _ in range(K):
    w = input()
    mp[w[0]].append(w)

for v in mp.values():
    v.sort()

idx = defaultdict(list)

  for _ in range(N):
    c = input()
    print(mp[c][idx[c]])
    idx[c] = (idx[c] + 1) % len(mp[c])  

  