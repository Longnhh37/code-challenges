# coci19c5p1
from collections import defaultdict

n, m = map(int, input().split())
grid = [input() for _ in range(n)]

starts = defaultdict(list)

for i in range(n):
    row = grid[i]
    j = 0
    while j < m:   
        if row[j] == '*':
            starts[j].append(i)
            while j < m and row[j] == '*':
                j += 1
        else:
            j += 1

total = 0
for rows in starts.values():
    cnt = 1
    for i in range(1, len(rows)):
        if rows[i] != rows[i-1] + 1:
            cnt += 1
    total += cnt
    
print(total)
    
        


    
    









         