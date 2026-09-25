# coci15c2p1

n = int(input())
data = [input() for _ in range(n)]
t9_feature = input()

key_mp = {('a', 'b', 'c'): '2', ('d', 'e', 'f'): '3',
             ('g', 'h', 'i'): '4', ('j', 'k', 'l'): '5',
             ('m', 'n', 'o'): '6', ('p', 'q', 'r', 's'): '7',
             ('t', 'u', 'v'): '8', ('w', 'x', 'y', 'z'): '9'
             }

char_to_digit_key_map = {
    c: d
    for chars, d in key_mp.items()
    for c in chars
    }

correct = 0
for datum in data:
    pressed = ''.join([char_to_digit_key_map.get(ch) for ch in datum])
    if pressed == t9_feature:
        correct += 1

print(correct)