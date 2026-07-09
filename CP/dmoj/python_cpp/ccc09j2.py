bt = int(input())
np = int(input())
yp = int(input())
limit = int(input())

max_bt = limit // bt
max_np = limit // np
max_yp = limit // yp

cnt = 0
combs = []

for i in range(max_bt + 1):
    for j in range(max_np + 1):
        for k in range(max_yp + 1):
            total = bt * i + np * j + yp * k
            if total == 0:
                continue

            if total <= limit:
                cnt += 1
                combs.append((i, j, k))
            else:
                break


for i, j, k in combs:
    print(f"{i} Brown Trout, {j} Northern Pike, {k} Yellow Pickerel")
print(f"Number of ways to catch fish: {cnt}")
