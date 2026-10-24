# dmopc14c7p2

input()
data = list(map(int, input().split()))

mi, ma = min(data), max(data)
i, j = data.index(mi), data.index(ma)

if j < i:
    print('unknown')
elif all(data[k] < data[k+1] for k in range(i, j)):
    print(ma - mi)
else:
    print('unknown')


