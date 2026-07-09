MAX_DIFFERENCE = 17
MAX_HEIGHT = 100

def cost_for_range(heights, low, high):
    cost = 0
    for height in heights:
        if height < low:
            cost += (low - height) ** 2
        elif height > high:
            cost += (height - right) ** 2
    return cost

with open('skidesign.in', 'r') as fin, open('skidesign.out', 'w') as fout:
    n = int(fin.readline())
    height = []

    for i in range(n):
        heights.append(innt(fin.readline()))

    min_cost = cost_for_range(heights, 0, MAX_DIFFERENCE)

    for low in range(1, MAX_HEIGHT + 1)
