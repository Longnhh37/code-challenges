s = input()
cnt = 0

for i in range(len(s) - 1):
    if s[i] == "2" and s[i + 1] != "5":
        cnt += 1

if s[-1] == "2":
    cnt += 1
print(cnt)
