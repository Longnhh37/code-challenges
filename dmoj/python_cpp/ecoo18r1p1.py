# ecoo18r1p1

for _ in range(2):
    t, n = map(int, input().split())
    data = [input() for _ in range(n)]
    cnt = 0
    for i in data:
        if i == 'B':
            cnt += t
        if cnt == 0:
            cnt += 1
        cnt -= 1
        
    print(cnt)