# cco99p2
from collections import Counter


def ordinal(n: int) -> str:
    if 11 <= (n % 100) <= 13:
        suffix = "th"
    else:
        suffix = ["th", "st", "nd", "rd", "th"][min(n % 10, 4)]
    return f"{n}{suffix}"


def nth_common_words(counter, n):  # dense ranking - 1223 ranking
    items = sorted(counter.items(), key=lambda x: -x[1])

    res = []
    prev_freq = None
    rank = 0

    for i, (k, v) in enumerate(items):
        if v != prev_freq:
            rank = i + 1
        if rank == n:
            res.append(k)
        elif rank > n:
            break
        prev_freq = v

    return res


# main
dataset = int(input())

for t in range(dataset):
    lines, common_th = map(int, input().split())
    counter = Counter()

    for _ in range(lines):
        counter[input()] += 1

    out = nth_common_words(counter, common_th)

    print(f"{ordinal(common_th)} most common word(s):")
    if out:
        print(*out, sep="\n")

    if t != dataset - 1:
        print()

