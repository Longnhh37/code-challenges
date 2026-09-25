from bisect import bisect_left

with open("angry.in", "r") as fin:
    n = int(fin.readline())
    a = sorted(map(int, fin))

max_cnt = 0
for i in range(n):
    cnt = 1

    r = 1
    left = i
    while True:
        target = a[left] - r
        idx = bisect_left(a, target)
        if idx < left:
            left = idx
            cnt += 1
            r += 1
        else:
            break

    r = 1
    right = i
    while True:
        target = a[right] + r
        idx = bisect_left(a, target)
        if idx < n:
            right = idx
            cnt += 1
            r += 1
        else:
            break

    max_cnt = max(max_cnt, cnt)

with open("angry.out", "w") as fout:
    fout.write(str(max_cnt))
