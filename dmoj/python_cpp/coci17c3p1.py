from sys import stdin

N = int(input())
a = stdin.read().split()

j = 0
cnt = 0

for i in range(1, N):
    if a[i] == a[j]:
        continue
    else:
        cnt += 1
        j = i

print(cnt + 2)
