import sys


def check_lower(w):
    for ch in w:
        if not "a" <= ch <= "z":
            return False
    return True


input()
data = sys.stdin.read().split()

cnt = 0

for w in data:
    first = w[0]
    last = w[-1]

    if last in ".?!":
        if "A" <= first <= "Z":
            w = w[1:-2]
            cnt += 1 if check_lower(w) else 0
        print(cnt)
        cnt = 0
    else:  # not an ending word
        if "A" <= first <= "Z":
            if check_lower(w[1:]):
                cnt += 1
