# timus Vol 12, 2144 Cleaning Room
def main():
    box = []
    n = int(input())
    for _ in range(n):
        size, *arr = list(map(int, input().split()))
        if any(arr[i] > arr[i + 1] for i in range(size - 1)):
            print("NO")
            return
        box.append((arr[0], arr[-1]))

    sorted_box = sorted(box)
    if any(sorted_box[j][1] > sorted_box[j + 1][0] for j in range(len(sorted_box) - 1)):
        print("NO")
        return

    print("YES")


main()
