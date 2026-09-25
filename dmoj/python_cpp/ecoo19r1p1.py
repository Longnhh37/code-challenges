# ecoo19r1p1
from collections import Counter
import sys

lines = sys.stdin.read().splitlines()

for i in range(0, len(lines), 2):
    total_clean, n_events, days = map(int, lines[i].split())
    events = list(map(int, lines[i + 1].split()))
    
    events_by_day = Counter(events)
    
    clean = total_clean 
    laundry = 0
    
    for today in range(1, days + 1):
        
        if clean == 0:
            clean = total_clean
            laundry += 1
        
        if today in events_by_day:
            clean += events_by_day[today]
            total_clean += events_by_day[today]
            
        clean -= 1
        
    print(laundry)
    
    
    
    