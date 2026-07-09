# USACO 2017 2/bronze/1

with open('crossroad.in', 'r') as fin:
    n = int(fin.readline())
    for _ in range(n):
        i, j = fin.readline().split()
        d[i] += 1
    
    out = sum(v // 2 for k, v in d.items())

with open('crossroad.out', 'w') as fout:
    fout.write(str(out))

