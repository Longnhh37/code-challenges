# ccc18j3

n = list(map(int, input().split()))

for mid in range(5):
    arr = n.copy()
    arr.insert(mid, 0)
    
    # Left prefix
    C = [0] * len(arr)
    C[mid] = arr[mid]
    
    for i in range(mid-1, -1, -1):
        C[i] = C[i+1] + arr[i]
    for i in range(mid+1, len(arr)):
        C[i] = C[i-1] + arr[i]
    
    print(*C)