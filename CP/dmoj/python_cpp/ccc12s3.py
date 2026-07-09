from collections import Counter
import sys

N = int(input())
c = Counter(map(int, sys.stdin.read().rstrip().split()))

first = second = float("-inf")
for v in c.values():
    if v > first:
        second = first
        first = v
    elif first > v > second:
        second = v

most_freq = []
sec_freq = []
for k, v in c.items():
    if v == first:
        most_freq.append(k)
    elif v == second:
        sec_freq.append(k)

if len(most_freq) > 1:
    print(max(most_freq) - min(most_freq))
    sys.exit()
else:
    most_freq = most_freq[0]

if len(sec_freq) == 1:
    print(abs(most_freq - sec_freq[0]))
else:
    largest = float("-inf")
    for i in sec_freq:
        if abs(most_freq - i) > largest:
            largest = abs(most_freq - i)
    print(largest)
