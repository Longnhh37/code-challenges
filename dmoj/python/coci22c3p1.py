import sys

N, M = map(int, input().split())
a = list(map(int, input().split()))

avail = 0
for i in a:
    if i < 4:
        print("NE")
        sys.exit()

    if M > N:
        print("NE")
        sys.exit()

    avail += i // 4

if avail >= N:
    print("DA")
else:
    print("NE")
