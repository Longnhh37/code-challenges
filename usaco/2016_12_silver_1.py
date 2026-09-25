from bisect import bisect_left, bisect_right

with open("haybales.in", "r") as fin:
    N, Q = map(int, fin.readline().split())
    locations = sorted(map(int, fin.readline().split()))
    queries = [list(map(int, fin.readline().split())) for _ in range(Q)]


out = []
for query in queries:
    begin, end = query[0], query[1]
    idx_beg = bisect_left(locations, begin)
    idx_end = bisect_right(locations, end)
    num_in_range = idx_end - idx_beg
    out.append(num_in_range)

with open("haybales.out", "w") as fout:
    fout.write("\n".join(map(str, out)))
