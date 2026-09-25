# coci18c4p1
import sys

input_text = sys.stdin.read().splitlines()

has_wand = input_text[0]
duels = input_text[2:]

wand_owner_set = {has_wand}
# wand_owner_last = [has_wand]

for duel in duels:
    winner, loser = duel.split()
    if loser == has_wand:
        has_wand = winner
        wand_owners_set.add(winner)

print(has_wand)
print (len(wand_owner_set))
