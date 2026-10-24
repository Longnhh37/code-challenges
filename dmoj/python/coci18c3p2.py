import sys

n = int(input())
a = list(map(int, sys.stdin.read().split()))

min_interval = float("inf")

for i in range(n - 1):
    diff = abs(a[i] - a[i + 1])
    if diff < min_interval:
        min_interval = diff


print(min_interval)
