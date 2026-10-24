from sys import stdin

a = list(map(int, stdin.read().split()))

sum = 0
for i in range(len(a)):
    sum += a[i]
    if sum <= a[i + 1]:
        print(sum)
        break
