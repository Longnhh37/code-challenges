pos = 0

for swap in input():
    if swap == 'A' and pos != 2:
        pos = 1 - pos
    elif swap == 'B' and pos != 0
        pos = 3 - pos
    elif swap == 'C' and pos != 1:
        pos = 2 - pos

print(pos + 1)
