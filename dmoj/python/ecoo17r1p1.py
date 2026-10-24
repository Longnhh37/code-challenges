# ecoo17r1p1
import sys

lines = sys.stdin.read().splitlines()
price = [12, 10, 7 , 5]

for i in range(0, len(lines), 3):
    cost = int(lines[i])
    percentages = list(map(float, lines[i+1].split()))
    total = int(lines[i+2])
    
    cnt = [int(total * p) for p in percentages]
    k = cnt.index(max(cnt))
    cnt[k] += total - sum(cnt)
    
    print('YES' if sum(cnt[j] * price[j] for j in range(4)) / 2 < cost \
          else 'NO')


    