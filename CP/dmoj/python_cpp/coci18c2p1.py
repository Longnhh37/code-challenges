# coci18c2p1
import sys


def count_comebacks_prefix(A, B):
    events = []

    for t in A:
        events.append((t, 1))  # A scored -> 1
    for t in B:
        events.append((t, -1))  # B scored -> -1

    events.sort()

    score_feed = [x[1] for x in events]

    # prefix sum
    pref = []
    cur = 0
    for x in score_feed:
        cur += x
        pref.append(cur)

    comebacks = 0
    for i in range(1, len(pref) - 1):
        if (pref[i] == 0) and (pref[i - 1] * pref[i + 1] < 0):
            comebacks += 1

    return comebacks


def main():
    data = list(map(int, sys.stdin.read().split()))

    first_half = 2 * 12 * 60

    score_log_A = data[1 : data[0] + 1]
    score_log_B = data[data[0] + 2 :]

    # -2 input for scores of each team
    print(sum(1 for i in range(len(data)) if data[i] <= first_half) - 2)
    print(count_comebacks_prefix(score_log_A, score_log_B))


if __name__ == "__main__":
    main()
