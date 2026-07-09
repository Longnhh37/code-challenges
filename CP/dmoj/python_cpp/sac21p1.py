from sys import stdin

start, interval, sent = map(int, stdin.read().split())

for i in range(1, 4):
    if start + i * interval >= sent:
        print(start + i * interval)
        break
else:
    print("Who knows...")
