N, M = map(int, input().split())
banned = set()
for _ in range(M):
    u, v = map(int, input().split())
    if u > v:
        u, v = v, u
    banned.add((u, v))

cnt = 0

for i in range(1, N - 1):
    for j in range(i + 1, N):
        if (i, j) in banned:
            continue
        for k in range(j + 1, N + 1):
            if (i, k) in banned or (j, k) in banned:
                continue
            cnt += 1
print(cnt)
