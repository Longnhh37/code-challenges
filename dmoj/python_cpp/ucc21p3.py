slices = int(input())
want_beg, want_end = map(int, input().split())

sum = 0

for _ in range(int(input())):
    l, r = map(int, input().split())
    if l > want_end or r < want_beg:
        continue
    sum += -max(want_beg, l) + min(want_end, r) + 1

print(sum)
