from collections import defaultdict
import sys

N, Q = map(int, input().split())
d = defaultdict(set)

lines = sys.stdin.readlines()

for line in lines:
    cmd, pos, name = line.split()

    if cmd == "1":
        if name in d[pos]:
            print(1)
        else:
            print(0)
    else:
        d[pos].add(name)
