#ccc07j3
import sys

briefcase = [100, 500, 1000, 5000, 10000, 25000, 50000, 100000, 500000, 1000000]

lines = sys.stdin.read().splitlines()

opened_num = int(lines[0])
opened_list = [int(x) for x in lines[1:-1]]
offered = int(lines[-1])

total = sum(briefcase)
for i in opened_list:
    total -= briefcase[i - 1]

avg = total / (10 - opened_num)

print('deal' if offered > avg else 'no deal')

