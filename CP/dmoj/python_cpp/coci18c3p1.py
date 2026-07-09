# coci18c3p1

text = input();
pattern = "HONI"
idx = 0
count = 0


for ch in text:
    if ch == pattern[idx]:
        idx += 1
        if idx == len(pattern):
            idx = 0
            count += 1

print(count)