with open("diamond.in", "r") as fin:
    N, K = map(int, fin.readline().split())
    a = sorted(map(int, fin.read().split()))

    r = 0
    max_cnt = 0
    for i in range(N):
        while r < N and a[r] <= a[i] + K:
            r += 1

        max_cnt = max(max_cnt, r - i)

with open("diamond.out", "w") as fout:
    fout.write(str(max_cnt))
