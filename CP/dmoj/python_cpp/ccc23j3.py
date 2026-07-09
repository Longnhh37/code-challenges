N = int(input())

total = [0] * 5
for _ in range(N):
    s = input()
    for i in range(5):
        if s[i] == "Y":
            total[i] += 1

max = max(total)
ans = [j + 1 for j in range(5) if total[j] == max]
print(",".join(str(k) for k in ans))
