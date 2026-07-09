# coci20c2p1

n = int(input())
s = input()
     
grid = [['.' for _ in range(n)] for _ in range(300)]
    
current_row = 150
min_r, max_r = 152, 150
    
for i in range(n):
    c = s[i]
        
    if c == '+':
        grid[current_row][i] = '/'
        min_r = min(min_r, current_row)
        max_r = max(max_r, current_row)
        current_row -= 1
        
    elif c == '-':
        current_row += 1
        grid[current_row][i] = '\\'
        min_r = min(min_r, current_row)
        max_r = max(max_r, current_row)
        
    elif c == '=':
        grid[current_row][i] = '_'
        min_r = min(min_r, current_row)
        max_r = max(max_r, current_row)

for r in range(min_r, max_r + 1):
    print("".join(grid[r]))

