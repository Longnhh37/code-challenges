# ecoo15r1p1
import sys
from collections import defaultdict

lines = sys.stdin.read().splitlines()

cnt = defaultdict(int)

for line in lines:
    if line == "end of box":
        time = 0
        for color, c in cnt.items():
            if color == 'red':
                time += c * 16
            else:
                time += ((c + 6) // 7) * 13
        print(time)
        cnt.clear()
    
    else:
        cnt[line] += 1