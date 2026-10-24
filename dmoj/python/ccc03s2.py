N = int(input())

vows = "aeiou"

for _ in range(N):
    syls = []
    for _ in range(4):
        w = input().split()[-1].lower()

        for i in range(len(w) - 1, -1, -1):
            ch = w[i]
            if ch in vows:
                syl = w[i:]
                break
            syl = w

        syls.append(syl)

    if syls[0] == syls[1] and syls[1] == syls[2] and syls[2] == syls[3]:
        print("perfect")
    elif syls[0] == syls[1] and syls[2] == syls[3]:
        print("even")
    elif syls[0] == syls[2] and syls[1] == syls[3]:
        print("cross")
    elif syls[0] == syls[3] and syls[1] == syls[2]:
        print("shell")
    else:
        print("free")
