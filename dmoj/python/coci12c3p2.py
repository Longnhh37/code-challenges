N = int(input())

correct = input().split()
wan = input().split()

cnt = 0
order = [correct.index(wan[i]) for i in range(N)]

for i in range(N):
    for j in range(i + 1, N):
        if order[i] < order[j]:
            cnt += 1

print(f"{cnt}/{N * (N-1)//2}")
