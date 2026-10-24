import sys

it = iter(sys.stdin.read().split("\n"))

next(it)

a1 = list(map(int, next(it).split()))
a2 = list(map(int, next(it).split()))

max_a1, max_a2 = max(a1), max(a2)
min_a1, min_a2 = min(a1), min(a2)

for _ in range(int(next(it))):
    query, where = map(int, next(it).split())

    if query == 1:
        if a1[where - 1] >= max_a2:
            ans = a1[where - 1]
        else:
            ans = max_a2
    elif query == 2:
        if a1[where - 1] < min_a2:
            ans = min_a2
        else:
            ans = a1[where - 1]
    elif query == 3:
        if a2[where - 1] >= max_a1:
            ans = a2[where - 1]
        else:
            ans = max_a1
    else:  # query == 4
        if a2[where - 1] < min_a1:
            ans = min_a1
        else:
            ans = a2[where - 1]

    print(ans)
