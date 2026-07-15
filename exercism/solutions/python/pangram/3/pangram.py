def is_pangram(sentence):
    return len({char.lower() for char in sentence if 'a' <= char.lower() <= 'z'}) == 26