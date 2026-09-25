#coci17c1p1
from collections import defaultdict
import sys

input()
drawn = sys.stdin.read().split()
drawn = [int(x) for x in drawn]

till_21 = 21 - sum(drawn)

deck = defaultdict(int)

for _ in range(4):
    for i in range(2, 10):
        deck[i] += 1
    deck[10] += 4
    deck[11] += 1

for j in drawn:
    deck[j] -= 1
    
lower = sum(v for k, v in deck.items() if k <= till_21)
upper = sum(v for k, v in deck.items() if k > till_21)

print('DOSTA' if upper >= lower else 'VUCI')