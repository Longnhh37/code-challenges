# dmopc19c5p1

N, M = map(int, input().split())

items = set()
for _ in range(N):
    items.add(input())


cnt = 0
for i in range(M):
    needs = int(input())
    data = [input() for _ in range(needs)]
    cannot = False

    for item in data:
        if item not in items:
            cannot = True
            break

    if cannot:
        continue
    cnt += 1

print(cnt)
