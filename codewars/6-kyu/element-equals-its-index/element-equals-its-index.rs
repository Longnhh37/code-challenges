fn index_equals_value(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return -1;
    } else if arr.len() == 1 {
        return (arr[0] == 0) as i32 - 1;
    }
    
    let (mut l, mut r) = (0_usize, arr.len() - 1);
    let mut res = -1;
    
    while l < r {
        let m = l + (r - l) / 2;
        let mi = m as i32;
        
        if arr[m] == mi {
            res = mi;
            r = m;
        } else if arr[m] > mi {
            r = m;
        } else {
            l = m + 1;
        }
    }
    res
}
​