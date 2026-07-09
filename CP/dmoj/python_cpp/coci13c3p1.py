#coci13c3p1
k = int(input())

numA = 1
numB = 0

for _ in range(k):
    value = numA
    numA = numB
    numB += value

print(numA, numB)
        