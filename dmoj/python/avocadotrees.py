import sys

input = sys.stdin.readline

N, Q, H = map(int, input().split())

pref = [0] * (N + 1)
for i in range(1, N + 1):
    h, y = map(int, input().split())
    pref[i] = pref[i - 1]
    if h <= H:
        pref[i] += y

max_avocado = -1
for _ in range(Q):
    x, y = map(int, input().split())
    cur = pref[y] - pref[x - 1]
    if cur > max_avocado:
        max_avocado = cur

print(max_avocado)
