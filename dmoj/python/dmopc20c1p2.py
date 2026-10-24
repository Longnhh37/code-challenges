from sys import stdin

N, D = map(int, input().split())
trolleys = list(map(int, input().split()))
snaps = list(map(int, stdin.read().split()))

pref = [0] * (N + 1)
for i in range(N):
    pref[i + 1] = pref[i] + trolleys[i]

L, R = 0, N

for snap in snaps:
    mid = L + snap

    sum_left = pref[mid] - pref[L]
    sum_right = pref[R] - pref[mid]

    if sum_left >= sum_right:
        print(sum_left)
        L = mid
    else:
        print(sum_right)
        R = mid
