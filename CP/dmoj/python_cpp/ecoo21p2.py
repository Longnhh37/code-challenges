s = input()

cons = "CTG"

ans = ""
cur = ""
for i in range(len(s)):
    v = s[i]

    if not cur:
        cur += v
        continue

    if (v == "A" and cur[-1] in cons) or (v in cons and cur[-1] == "A"):
        cur += v
    else:
        ans += cur + " "
        cur = v

ans += cur
print(ans)
