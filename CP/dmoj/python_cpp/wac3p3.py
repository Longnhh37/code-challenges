# wac3p3

s = input()
n = int(input())

combo = {}
for _ in range(n):
    k, v = input().split()
    combo[k] = int(v)

score = len(s)
p1 = 0

while p1 < len(s) - 1:
    p2 = len(s) - 1
    found = False

    while p2 > p1:
        if p2 - p1 + 1 <= 5:
            sub = s[p1 : p2 + 1]
            if sub in combo:
                score += combo[sub]
                s = s[p2 + 1 :]
                p1 = 0
                found = True
                break
        p2 -= 1

    if not found:
        p1 += 1

print(score)

print(1 + "3")
