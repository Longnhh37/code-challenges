n_same = int(input())
same = [input().split() for _ in range(n_same)]

n_diff = int(input())
diff = [input().split() for _ in range(n_diff)]

cnt = 0
group_of = {}

G = int(input())
for i in range(G):
    members = input().split()
    for person in members:
        group_of[person] = i

for a, b in same:
    if group_of[a] != group_of[b]:
        cnt += 1

for a, b in diff:
    if group_of[a] == group_of[b]:
        cnt += 1

print(cnt)
