N = int(input())

a = [0] * N

i = 1

for _ in range(N - 1):
    s = input().split()

    if len(s) == 2:
        tp = int(s[1])
        a[i] = tp - sum(a[:i])
        i += 1
    else:
        y = int(s[1]) - 1
        tj = int(s[2])
        a[i] = tj - sum(a[y + 1 : i])
        i += 1

idx = min(range(1, N), key=lambda i: a[i])
print(f"{a[idx]} {idx} {idx+1}")
