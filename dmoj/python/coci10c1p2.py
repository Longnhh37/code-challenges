N = int(input())

streak = [0] * 5
best_streak = -1
best_grade = 6

for _ in range(N):
    i, j = map(int, (input().split()))
    i -= 1
    j -= 1

    for k in range(5):
        if k == i or k == j:
            streak[k] += 1
        else:
            streak[k] = 0

    cur_max = max(streak)

    if cur_max > best_streak:
        best_streak = cur_max
        best_grade = min(k for k in range(5) if streak[k] == cur_max) + 1
    elif cur_max == best_streak:
        cur_grade = min(k for k in range(5) if streak[k] == cur_max) + 1
        best_grade = min(best_grade, cur_grade)

print(best_streak, best_grade)
