# USACO2019/2/Bronze/2/Revegetation

with open("revegetate.in", "r") as fin, open("revegetate.out", "w") as fout:
    n, m = list(map(int, fin.readline().split()))
    data = []
    for _ in range(m):
        data.append(tuple(map(int, fin.readline().split())))

    pastures = []
    cows = {}

    for i in range(n):
        SEED = [1,2,3,4]
        pasture = i + 1
        find_cow = []
        
        for j in range(m):
            if data[j][0] == pasture or data[j][1] == pasture:
                find_cow.append(j)
        
        if pasture == 1:
            cows[tuple(find_cow)] = 1
            pastures.append((1, 1))
            continue
        
        for k in find_cow:
            for key in cows:
                if k in key:
                    if cows[key] in SEED:
                        SEED.remove(cows[key])
        
        pastures.append((pasture, min(SEED)))
        if len(find_cow) > 0:
            cows[tuple(find_cow)] = min(SEED)

    output = ''
    for i in pastures:
        output += str(i[1])

    fout.write(output)
    
    
    
