# coci08c3p2

vowels = set('aiueo')

text = input()

result = []
i = 0

while i < len(text):
    if text[i] in vowels and text[i+1] == 'p' and text[i+2] == text[i]:
        result.append(text[i])
        i += 3
    else:
        result.append(text[i])
        i += 1 

print(''.join(result))
