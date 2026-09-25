N = int(input())
# (amount, capacity, original index)
arr = []
for i in range(N):
    x, y = map(int, input().split())
    arr.append((x, y, i))

arr.sort(key=lambda t: (-t[1], -t[0]))

i = 0
j = N - 1

while i < j:
    cur, cap, _ = arr[i]
    to_pour, _, _ = arr[j]

    need = cap - cur

    if to_pour == 0:
        j -= 1
        continue

    if need == 0:
        i += 1
        continue

    pour = min(to_pour, need)

    arr[i] = (cur + pour, cap, arr[i][2])
    arr[j] = (to_pour - pour, arr[j][1], arr[j][2])

out = [0] * N
for cur, _, idx in arr:
    out[idx] = cur

print(out.count(0))
print(*out)
