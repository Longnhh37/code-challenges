# coci13c2p2

def count_edge(grid, row, col):
    biggest_row = len(grid) - 1
    biggest_col = len(grid[0]) - 1
    cnt = 0
    
    # Right
    if col < biggest_col and grid[row][col+1] == 'o':
        cnt += 1
     
    # Right up
    if col < biggest_col and row > 0 and grid[row-1][col+1] == 'o':
        cnt += 1 
    
    # Right down
    if col < biggest_col and row < biggest_row and grid[row+1][col+1] == 'o':
        cnt += 1
    
    # Left
    if col > 0 and grid[row][col-1] == 'o':
        cnt += 1
    
    # Left up
    if col > 0 and row > 0 and grid[row-1][col-1] == 'o':
        cnt += 1
    
    # Left down
    if col > 0 and row < biggest_row and grid[row+1][col-1] == 'o':
        cnt += 1
        
    # Up 
    if row > 0 and grid[row-1][col] == 'o':
        cnt += 1 
    
    # Down
    if row < biggest_row and grid[row+1][col] == 'o':
        cnt += 1 
        
    return cnt

               
def best_degree(grid):
    best_row = -1
    best_col = -1
    most = 0
    
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == '.':
                edge_cnt = count_edge(grid, row, col)
                if edge_cnt > most:
                    most = edge_cnt
                    best_row = row
                    best_col = col
                    
    return ((best_row, best_col))
        
# Main
m, n = list(map(int, input().split())) # row, col

grid = []
for _ in range(m):
    grid.append(list(input()))

best_degree_list = best_degree(grid)
best_row, best_col = best_degree_list

if best_row >= 0 and best_col >= 0:
    grid[best_row][best_col] = 'o'

edge_cnt = 0
for i in range(m):
    for j in range(n):
        if grid[i][j] == 'o':
            edge_cnt += count_edge(grid, i, j)

edge_cnt //= 2
print(edge_cnt)
