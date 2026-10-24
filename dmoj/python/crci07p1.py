# crci07p1

n = int(input())
dataset = []
for _ in range(n):
    data = tuple(map(int, input().split()))
    dataset.append(data)
dataset.sort()

total = dataset[0][0] * 2

for i in range(1, n):
    y, x1, x2 = dataset[i]
    total += y * 2
    
    for j in range(i-1, -1, -1):
        prev_y, prev_x1, prev_x2 = dataset[j]
        if prev_x1 <= x1 < prev_x2:
            total -= prev_y
            break
    
    for k in range(i-1, -1, -1):
        prev_y, prev_x1, prev_x2 = dataset[k]
        if prev_x1 < x2 <= prev_x2:
            total -= prev_y
            break
    

print(total)
 