R, C = map(int, input().split())

x_pos = [False] * (C + 1)
y_pos = [False] * (R + 1)

for i in range(R):
    row = input()
    for j, ch in enumerate(row):
        if ch == "X":
            x_pos[j + 1] = True
            y_pos[i + 1] = True

Q = int(input())
for _ in range(Q):
    x, y = map(int, input().split())
    if x_pos[x] or y_pos[y]:
        print("Y")
    else:
        print("N")
