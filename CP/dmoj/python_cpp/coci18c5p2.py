from collections import Counter

N = int(input())
fac = list(map(int, input().split()))
req = list(map(int, input().split()))

diff = [req[i] - fac[i] for i in range(N)]
c = Counter(diff)

print(max(c.values()))
