# ccc11s2

n = int(input())
student_answer = [input() for _ in range(n)]
correct_answer = [input() for _ in range(n)]

count = 0

for i in range(n):
    if student_answer[i] == correct_answer[i]:
        count += 1

print(count)