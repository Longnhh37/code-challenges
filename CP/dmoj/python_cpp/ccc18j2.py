#ccc18j2
input()
yesterday = input()
today = input()

result = sum(1
             for y, t in zip(yesterday, today)
             if y == 'C' and t == 'C')

print(result)

