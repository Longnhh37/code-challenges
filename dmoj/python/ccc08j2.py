# ccc08j2
playlist = ['A', 'B', 'C', 'D', 'E']

def but1():
    song = playlist.pop(0)
    playlist.append(song)

def but2():
    song = playlist.pop()
    playlist.insert(0, song)

def but3():
    playlist[0], playlist[1] = playlist[1], playlist[0]

while True:
    b = int(input())
    n = int(input())
    
    match b:
        case 1:
            for _ in range(n):
                but1()
        case 2:
            for _ in range(n):
                but2()
        case 3:
            for _ in range(n):
                but3()
        case 4:
            break

print(' '.join(playlist))
