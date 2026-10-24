from collections import Counter

K = int(input())
d = {}

for _ in range(K):
    ch, point, limit = input().split()
    d[ch] = (int(point), int(limit))

max_score = -1

for _ in range(int(input())):
    c = Counter(input())
    score = 0

    for ch, freq in c.items():
        point, limit = d.get(ch, (0, 0))
        if freq <= limit:
            score += freq * point
        else:
            score = 0
            break

    if score > max_score:
        max_score = score

print(max_score)
