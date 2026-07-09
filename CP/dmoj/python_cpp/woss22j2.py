N = int(input())
a = list(map(int, input().split()))

inc = dec = 1
max_inc = max_dec = 0

for i in range(1, N):
    if a[i] > a[i - 1]:
        inc += 1
    else:
        inc = 1

    if inc > max_inc:
        max_inc = inc

    if a[i] < a[i - 1]:
        dec += 1
    else:
        dec = 1

    if dec > max_dec:
        max_dec = dec

print(max_inc)
print(max_dec)
