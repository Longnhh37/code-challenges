from collections import defaultdict

with open("whereami.in", "r") as fin:
    n = int(fin.readline())
    s = fin.readline()

end = None
d = defaultdict(int)

for i in range(1, n):
    not_unique = False
    begin = 0
    end = i

    while end <= n:
        w = s[begin:end]
        d[w] += 1
        if d[w] == 2:
            not_unique = True
            break

        begin += 1
        end += 1

    if not_unique:
        d.clear()
        continue

    break

with open("whereami.out", "w") as fout:
    fout.write(str(i))
