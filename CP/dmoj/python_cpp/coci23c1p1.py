import sys

lines = sys.stdin.read().splitlines()

board = []

for line in lines:
    row = []
    if line.startswith("|"):
        for ch in line:
            if ch.isdigit() or ch == ".":
                row.append(ch)
        board.append(row)


for i in range(9):
    seen = set()
    for j in range(9):
        if board[i][j] == ".":
            continue
        if board[i][j] in seen:
            print("GRESKA")
            sys.exit()
        seen.add(board[i][j])

for j in range(9):
    seen = set()
    for i in range(9):
        if board[i][j] == ".":
            continue
        if board[i][j] in seen:
            print("GRESKA")
            sys.exit()
        seen.add(board[i][j])


for bi in range(0, 9, 3):
    for bj in range(0, 9, 3):
        seen = set()
        for i in range(3):
            for j in range(3):
                val = board[bi + i][bj + j]
                if val == ".":
                    continue
                if val in seen:
                    print("GRESKA")
                    sys.exit()
                seen.add(val)

print("OK")
