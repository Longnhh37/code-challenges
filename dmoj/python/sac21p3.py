N, P = map(int, input().split())

max = -(10**9)
min = 10**9

for i in range(N):
    name, m, cs, e = input().split()
    m, cs, e = float(m), float(cs), float(e)
    s = 4 * m ** (1 / 2) + 3 * cs**P - 4 * e

    if s > max:
        max = s
        ans_max = [name, s]

    if s < min:
        min = s
        ans_min = [name, s]

print(*ans_max)
print(*ans_min)
