H = int(input())
A = int(input())
S = int(input())

ans = min(H, A) - S
if ans <= 0:
    print(0)
else:
    print(ans)
