# ecoo18r1p2
from collections import defaultdict

for _ in range(10):
    d = defaultdict(list)

    n = int(input())
    
    for i in range(n):
        route = list(map(int, input().split()))
        route_id = route[0]
        route_min = min(route[2:])
        d[route_min].append(route_id)
    
    s1 = min(d)
    s2 = d[min(d)]
    s2.sort()

    print(str(s1) + ' {' + ','.join(map(str, s2)) + '}')


 
        
