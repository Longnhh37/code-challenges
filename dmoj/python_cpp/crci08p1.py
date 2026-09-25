s = input()

split = []
cur = s[0]

for i in range(1, len(s)):
    v = s[i]

    if "a" <= v <= "z":
        cur += v
    elif "A" <= v <= "Z":
        split.append(cur)
        cur = v

split.append(cur)

cnt = 0
for i in range(len(split) - 1):
    chunk = len(split[i])
    while chunk % 4 != 0:
        chunk += 1
        cnt += 1


print(cnt)
