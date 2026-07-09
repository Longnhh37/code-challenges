N = int(input())
cnt = 0

for _ in range(N):
    score = int(input())
    foul = int(input())
    total = score * 5 - foul * 3
    if total > 40:
        cnt += 1

if cnt < N:
    print(cnt)
else:
    print(f"{cnt}+")
