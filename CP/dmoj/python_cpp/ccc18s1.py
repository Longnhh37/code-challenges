#ccc18s1
import sys

input()
lines = sys.stdin.read().splitlines()

road = sorted(int(line) for line in lines)

neighbor = [((road[i+1] - road[i-1]) / 2) for i in range(1, len(road)-1)]

print(f"{min(neighbor):.1f}")


    
