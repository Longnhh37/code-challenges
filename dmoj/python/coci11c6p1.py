N, M = map(int, input().split())
a = sorted(map(int, input().split()), reverse=True)

max_point = 0

for i in range(N):
    for j in range(1 + i, N):
        for k in range(1 + j, N):
            total = a[i] + a[j] + a[k]
            if total <= M and total > max_point:
                max_point = total

print(max_point)
