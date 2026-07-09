N, Q = map(int, input().split())
intervals = [tuple(map(int, input().split())) for _ in range(Q)]
intervals.sort()

res = []
for l, r in intervals:
    if not res or l > res[-1][1]:
        res.append((l, r))
    else:
        res[-1] = (res[-1][0], max(res[-1][1], r))

blue = 0
for l, r in res:
    blue += r - l
purple = N - blue

print(f"{purple} {blue}")
