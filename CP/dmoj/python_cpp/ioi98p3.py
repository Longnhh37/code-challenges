N = int(input())
C = int(input())
lamps_on = list(map(int, input().split()))
lamps_on.pop()
lamps_off = list(map(int, input().split()))
lamps_off.pop()

possible = set()

for b1 in (0, 1):
    for b2 in (0, 1):
        for b3 in (0, 1):
            for b4 in (0, 1):
                if b1 + b2 + b3 + b4 > C:
                    continue
                if (b1 + b2 + b3 + b4) % 2 != (C % 2):
                    continue

                lamps = [1] * N
                if b1 == 1:
                    lamps = [0] * N
                if b2 == 1:
                    for i in range(0, N, 2):
                        lamps[i] ^= 1
                if b3 == 1:
                    for i in range(1, N, 2):
                        lamps[i] ^= 1
                if b4 == 1:
                    for i in range(0, N, 3):
                        lamps[i] ^= 1

                ok = True
                for on in lamps_on:
                    if lamps[on - 1] == 0:
                        ok = False
                for off in lamps_off:
                    if lamps[off - 1] == 1:
                        ok = False

                if ok:
                    possible.add("".join(map(str, lamps)))

if not possible:
    print("IMPOSSIBLE")
else:
    for config in possible:
        print(config)
