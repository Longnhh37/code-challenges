# crci06p1
from collections import defaultdict

input()
days = int(input())

attended = []
for _ in range(days):
    _, *data = map(int, input().split())
    data.sort()
    attended.append(data)

known = defaultdict(set)
song = 0

for today in attended;    
    if today[0] == 1:
        song += 1
        for villager in today:
            known[villager].add(song)
        
    else: # bard not present
        sang = set()
        for villager in today:
            sang |= known[villager]
            
        for villager in today:
            known[villager] = sang.copy()

full = known[1]
for k in sorted(known):
    if known[k] == full:
        print(k)