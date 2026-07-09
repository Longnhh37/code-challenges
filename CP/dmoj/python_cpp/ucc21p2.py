N = int(input())
a = list(map(int, input().split()))

max = 0
cur = 0

for i in range(N):
    if a[i] % 2 == 0:
        cur += a[i]
        if cur > max:
            max = cur
    else:
        if cur > max:
            max = cur
        cur = 0

print(max)
