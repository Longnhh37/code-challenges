s = input()

ans = s
for i in range(len(s) - 1):
    for j in range(i + 1, len(s)):
        left = s[:i][::-1]
        if len(left) == 0:
            continue
        mid = s[i:j][::-1]
        right = s[j:][::-1]
        all = left + mid + right

        if all < ans:
            ans = all

print(ans)
