import sys

n = int(input())
move = {}
owner = {}
for _ in range(n):
    person, old, new = input().split()
    move[old] = new
    owner[old] = person

all_old = set(move.keys())
all_new = set(move.values())
starts = list(all_new - all_old)

if not starts:
    print("Impossible")
    sys.exit()

if not (all_new & all_old):
    for x in owner.values():
        print(x)
    sys.exit()

ans = []
reverse_move = {v: k for k, v in move.items()}

for start in starts:
    cur = reverse_move[start]
    ans.append(owner[cur])
    while cur in reverse_move:
        cur = reverse_move[cur]
        ans.append(owner[cur])

if len(ans) < n:
    print("Impossible")
    sys.exit()

for x in ans:
    print(x)
