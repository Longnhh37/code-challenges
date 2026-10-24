#ccc99s1
import sys
    
plays = sys.stdin.read().splitlines()
    
# cards set up
cards_left = 52
HIGH = {
    'jack': 1,
    'queen': 2,
    'king': 3,
    'ace': 4
}
    
# game setup
player = "AB"
current = 0 # 0 -> A, 1 -> B 
score = [0, 0]

# streak setup
owner = None
point = 0
left = 0

# main game
for card in plays:
    cards_left -= 1
    
    if card in HIGH: 
        v = HIGH[card]
        if cards_left >= v: 
            owner = current
            left = v
            point = v
        else:
            current = 1 - current
            continue
    
    else: # low cards
        if left > 0:
            left -= 1
            if left == 0:
                score[owner] += point
                print(f"Player {player[owner]} scores {point} point(s).")
                
    # switch turn
    current = 1 - current
    
# game ended
for p in (0, 1):
    print(f"Player {player[p]}: {score[p]} point(s).")
        
            
        
        
    
    
    
        






