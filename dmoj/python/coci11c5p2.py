import sys

N, M = map(int, input().split())
a = list(map(int, sys.stdin.read().split()))
a.sort(reverse=True)

pref = [0] * (N + 1)
for i in range(N):
    pref[i + 1] = pref[i] + a[i]

h_diff_prev = 0
ans = 0

for i in range(1, N):
    h_diff = pref[i] - i * a[i]

    if h_diff > M:
        ans = a[i - 1] - (M - h_diff_prev + i - 1) // i
        break
    elif h_diff == M:
        ans = a[i]
        break

    h_diff_prev = h_diff

print(ans)
