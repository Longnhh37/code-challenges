# ecoo13r1p1
import sys

lines = sys.stdin.read().split()

current_num = int(lines[0])
actions = lines[1:]
late_count = 0
served = 0

for i in range(len(actions)):
    if current_num > 999:
        current_num = 1
        
    if actions[i] == 'TAKE':
        late_count += 1
        current_num += 1
    elif actions[i] == 'SERVE':
        served += 1
    elif actions[i] == 'CLOSE':
        print(late_count, late_count - served, current_num)
        late_count = 0
        served = 0