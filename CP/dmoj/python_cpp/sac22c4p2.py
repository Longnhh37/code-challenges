N, Q = map(int, input().split())
a = [0] * (N + 1)

map = {"square": 1, "circle": 2, "triangle": 3}

for _ in range(Q):
    query = input()
    pos = int(query[-1])
    shape = query[4:-2]
    cmd = query[:3]

    if cmd == "set":
        a[pos] = map[shape]
    else:  # cmd == 'get'
        if a[pos] == map[shape]:
            print(1)
        else:
            print(0)
