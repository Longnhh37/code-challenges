def is_pangram(sentence):
    return len(set([char.lower() for char in sentence if 'a' <= char.lower() <= 'z'])) == 26

