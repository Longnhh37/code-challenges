# USACO 2020/1/Brionze/Word Processor

with open('word.in', 'r') as fin, open('word.out', 'w') as fout:
    n, k = map(int, fin.readline().split())
    words = fin.readline().split()
    
    line = ''
    chars_on_line = 0
    
    for word in words:
        if chars_on_line + len(word) <= k:
            line = line + word + ' '
            chars_on_line += len(word)
        else:
            fout.write(line[:-1] + '\n')
            line = word + ' '
            chars_on_line = len(word)
    
    fout.write(line[:-1] + '\n')
    
