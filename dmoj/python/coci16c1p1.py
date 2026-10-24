# coci16c1p1
data_per_month = int(input())
month = int(input())
data_use = [int(input()) for _ in range(month)]

result = data_per_month * (month + 1) - sum(data_use)

print(result)