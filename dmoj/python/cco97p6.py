s1 = input().strip()
s2 = input().strip()

n = len(s1)
m = len(s2)

best_score = -(10**9)
best_shift = 0

for shift in range(-m, n + 1):
    score = 0

    for pos in range(max(n, m + abs(shift))):
        i = pos
        j = pos - shift

        if 0 <= i < n and 0 <= j < m:
            if s1[i] == s2[j]:
                score += 3
            else:
                score -= 1
        elif 0 <= i < n or 0 <= j < m:
            score -= 1

    if score > best_score:
        best_score = score
        best_shift = shift

aligned1 = []
aligned2 = []

L = max(n, m + abs(best_shift))

for pos in range(L):
    i = pos
    j = pos - best_shift

    if 0 <= i < n:
        aligned1.append(s1[i])
    else:
        aligned1.append("-")

    if 0 <= j < m:
        aligned2.append(s2[j])
    else:
        aligned2.append("-")

print(best_score)
print("".join(aligned1))
print("".join(aligned2))
