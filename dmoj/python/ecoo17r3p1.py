#ecoo17r3p1
for _ in range(10):
    count = 0
    
    c, r = map(int, input().split())
    table = [list(map(int, input().split())) for _ in range(r)]

    for s in map(sum, table):
        if s % 13 == 0:
            count += s // 13
    
    for s in map(sum, zip(*table)):
        if s % 13 == 0:
            count += s // 13
            
    print(count)