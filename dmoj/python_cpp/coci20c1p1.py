def dir_check(i, j, grid):
    d = 0
    while True:
        cur = grid[i][j]
        if cur == "." or cur == "o":
            return None

        d += 1
        if cur == ">":
            j += 1
        elif cur == "<":
            j -= 1
        elif cur == "^":
            i -= 1
        elif cur == "v":
            i += 1
        else:  # cur = 'x'
            return d


r, s = map(int, input().split())
grid = [input() for _ in range(r)]

for x0 in range(r):
    if "o" in grid[x0]:
        y0 = grid[x0].index("o")
        break

dx = [-1, 1, 0, 0]
dy = [0, 0, -1, 1]
dirChar = ["N", "S", "W", "E"]


out = []
for k in range(4):
    nx = x0 + dx[k]
    ny = y0 + dy[k]
    d = dir_check(nx, ny, grid)
    if d is not None:
        out.append((d, dirChar[k]))

if not out:
    print(":(")
else:
    print(":)")
    print(min(out)[1])
