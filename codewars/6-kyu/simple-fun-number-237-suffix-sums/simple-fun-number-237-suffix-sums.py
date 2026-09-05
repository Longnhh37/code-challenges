def suffix_sums(arr):
    sum = 0
    res = []
    
    for n in arr[::-1]:
        sum += n
        res.append(sum)
        
    return res[::-1]