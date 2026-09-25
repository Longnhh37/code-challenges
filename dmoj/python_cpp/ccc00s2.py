# ccc00s2
import sys

n = int(input())
stream = [int(input()) for _ in range(n)]
data = list(map(int, sys.stdin.read().split()))

while True:
    if data[0] == 77:
        break

    elif data[0] == 99:
        batch = data[1:3]
        del data[0:3]

        idx = batch[0] - 1
        per = batch[1]

        left = stream[idx] * per / 100
        right = stream[idx] - left

        stream[idx] = left
        stream.insert(idx + 1, right)

    elif data[0] == 88:
        batch = data[1]
        del data[0:2]

        idx = batch - 1
        stream[idx + 1] += stream[idx]
        stream.pop(idx)

print(*stream)

