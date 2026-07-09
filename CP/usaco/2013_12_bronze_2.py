# 2013_12_bronze_2
from bisect import bisect_left, bisect_right

with open("baseball.in", "r") as fin:
    n = int(fin.readline())
    a = sorted(map(int, fin))

cnt = 0

for i in range(n):
    for j in range(i + 1, n):
        d = a[j] - a[i]
        low = a[j] + d
        high = a[j] + d * 2
        left = bisect_left(a, low)
        right = bisect_right(a, high)
        cnt += right - left

with open("baseball.out", "w") as fout:
    fout.write(str(cnt))


def add(a: int, b: int) -> int:
    return a + b
