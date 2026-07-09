with open("shell.in", "r") as fin:
    n = int(fin.readline())
    a = [list(map(int, fin.readline().split())) for _ in range(n)]

max_score = 0
for i in range(3):
    cnt = 0
    cups = [0] * 3
    cups[i] = 1
    for j in range(n):
        if cups[a[j][0] - 1] == 1 or cups[a[j][1] - 1] == 1:
            cups[a[j][0] - 1] = 1 - cups[a[j][0] - 1]
            cups[a[j][1] - 1] = 1 - cups[a[j][1] - 1]
        if cups[a[j][2] - 1] == 1:
            cnt += 1
    max_score = max(max_score, cnt)

with open("shell.out", "w") as fout:
    fout.write(str(max_score))
