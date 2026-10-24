import sys

input = sys.stdin.readline

N, M = map(int, input().split())
states = list(map(int, input().split()))
toggles = list(map(int, input().split()))

cur_on = sum(states)

if cur_on == 0:
    print(0)
    sys.exit()

for i in range(M):
    idx = toggles[i] - 1
    states[idx] ^= 1
    cur_on += 1 if states[idx] else -1

    if cur_on <= i + 1:
        print(i + 1)
        break
else:
    print(cur_on)
