N, L = map(int, input().split())

sig = [list(map(int, input().split())) for _ in range(N)]

time = 0
step = 0

i = 0

while step < L:
    if i == N or step != sig[i][0]:
        time += 1
        step += 1
        continue

    r, g = sig[i][1], sig[i][2]
    time_left = time % (r + g)

    if time_left < r:
        time += r - time_left

    i += 1

print(time)
