_, num_relatives = map(int, input().split())
color = list(map(int, input().split()))
wants = [tuple(map(int, input().split())) for _ in range(num_relatives)]

first_pos = {}
last_pos = {}

for i, c in enumerate(color):
    if c not in first_pos:
        first_pos[c] = i
    last_pos[c] = i

max_length = 0

for a, b in wants:
    if a in first_pos and b in last_pos:
        i_beg = first_pos[a]
        i_end = last_pos[b]
        if i_end >= i_beg:
            max_length = max(max_length, i_end - i_beg + 1)


print(max_length)
