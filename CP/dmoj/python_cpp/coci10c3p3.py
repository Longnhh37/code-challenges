N, M, K = map(int, input().split())

best = [0.0] * (N + 1)

for _ in range(M):
    data = input().split()

    i = 0
    while i <= N * 2 - 2:
        idx = int(data[i])
        v = float(data[i + 1])

        if v > best[idx]:
            best[idx] = v

        i += 2

best.sort(reverse=True)

total = 0
for i in range(K):
    total += best[i]

print(round(total, 1))
