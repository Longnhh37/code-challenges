def smaller(arr):
    res = [0 for _ in range(len(arr))]
    for i in range(len(arr) - 1):
        cur = arr[i];
        cnt = 0
        for j in range(i + 1, len(arr)):
            if cur > arr[j]:
                cnt += 1
        res[i] = cnt
    return res
            