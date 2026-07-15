def is_pangram(sentence):
    char = set([c.lower() for c in sentence if 'a' <= c.lower() <= 'z'])
    return len(char) == 26
