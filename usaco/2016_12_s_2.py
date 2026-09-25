from collections import Counter

with open('citystate.in', 'r') as fin:
    n = fin.readline()
    n = int(n)
    
    for _ in range(n):
        s = fin.readline()
        pair = ((s[0:2], s[-2]))
        
        d = Counter()
        d[pair] += 1
     
    total_cnt = 0
    for k, v in d.items():
        cnt = d.get(k) * d.get(tuple([k[1], k[0]]), 1)
        cnt //= 2

        total_cnt += cnt

with open('citystate.out', 'w') as fout:
    fout.write(str(total_cnt))