from bisect import bisect_left, bisect_right

N, H = map(int, input().split())

floor = []
ceil = []

for i in range(N):
    x = int(input())
    if i % 2 == 0:
        floor.append(x)
    else:
        ceil.append(x)


floor.sort()
ceil.sort()

min_hit = N + 1
cnt = 0

for i in range(1, H + 1):
    # floor: length >= i
    hit_floor = len(floor) - bisect_left(floor, i)

    # ceil: length > H - i
    hit_ceil = len(ceil) - bisect_right(ceil, H - i)

    hit = hit_floor + hit_ceil
    if hit < min_hit:
        min_hit = hit
        cnt = 1
    elif hit == min_hit:
        cnt += 1

print(min_hit, cnt)
