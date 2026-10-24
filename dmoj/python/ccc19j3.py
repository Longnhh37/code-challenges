# ccc19j3
from itertools import groupby
import sys


input()
lines = sys.stdin.read().splitlines()


for line in lines:
    sequence = []
    for ch, g in groupby(line):
        sequence.append(f'{len(list(g))} {ch}')
    print(' '.join(sequence))
        

 
    