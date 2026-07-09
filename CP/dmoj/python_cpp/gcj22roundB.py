n = int(input())

for i in range(n):
    c1, m1, y1, k1 = map(int, input().split())
    c2, m2, y2, k2 = map(int, input().split())
    c3, m3, y3, k3 = map(int, input().split())

    min_c = min(c1, c2, c3)
    min_m = min(m1, m2, m3)
    min_y = min(y1, y2, y3)
    min_k = min(k1, k2, k3)

    if min_c + min_m + min_y + min_k < 10**6:
        print(f"Case #{i + 1}: IMPOSSIBLE")
    else:
        need = 10**6

        if need <= min_c:
            print(f"Case #{i+1}: {need} 0 0 0")
            continue
        need -= min_c

        if need <= min_m:
            print(f"Case #{i+1}: {min_c} {need} 0 0")
            continue
        need -= min_m

        if need <= min_y:
            print(f"Case #{i+1}: {min_c} {min_m} {need} 0")
            continue
        need -= min_y

        print(f"Case #{i+1}: {min_c} {min_m} {min_y} {need}")

