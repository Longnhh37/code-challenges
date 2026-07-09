# ecoo19r2p1

for _ in range(10):
    n = int(input())
    unique = set()
    
    for i in range(n):
        s = input().strip().lower()
        left, right = s.split('@', 1)
        
        left = left.split('+', 1)[0]
        left = left.replace('.', '')
        
        unique.add(left + '@' + right)

    print(len(unique))
    
    
        
        