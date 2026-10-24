import sys

a = sys.stdin.read().split()
a.pop()
ans = []

for i in a:
    first = int(i[0])
    second = int(i[1])
    dir = int(i[0]) + int(i[1])
    if dir % 2 == 1:
        ans.append(("left", i[2:]))
    elif dir == 0:
        ans.append((ans[-1][0], i[2:]))
    else:
        ans.append(("right", i[2:]))

for j, k in ans:
    print(f"{j} {k}")
