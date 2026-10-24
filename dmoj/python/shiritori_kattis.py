import sys

N = int(input())
a = sys.stdin.read().rstrip().split("\n")

turn = 1
spoken = set()
spoken.add(a[0])

for i in range(1, N):
    prev_ch = a[i - 1][-1]
    cur_ch = a[i][0]
    w = a[i]

    if w in spoken:
        print(f"Player {turn + 1} lost ")
        sys.exit()
    else:
        spoken.add(w)

    if prev_ch == cur_ch:
        turn ^= 1
        continue

    print(f"Player {turn + 1} lost ")
    sys.exit()

print("Fair Game")
