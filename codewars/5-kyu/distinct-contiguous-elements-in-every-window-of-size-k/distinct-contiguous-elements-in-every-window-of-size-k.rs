use std::collections::HashMap;
​
fn count_contiguous_distinct(k: u32, arr: &[i32]) -> Vec<u32> {
    let k = k as usize;
    let mut res = Vec::with_capacity(arr.len() - k + 1);
    let mut seen = HashMap::new();
    let mut cnt = 0;
    
    for (i, &n) in arr.iter().enumerate() {
        let e1 = seen.entry(n).or_insert(0);
        *e1 += 1;
        if *e1 == 1 {
            cnt += 1;
        }
        
        if i < k - 1 {
            continue;
        } else if i == k - 1 {
            res.push(cnt)
        } else {
            let e2 = seen.get_mut(&arr[i - k]).unwrap();
            *e2 -= 1;
            if *e2 == 0 {
                cnt -= 1;
            }
            res.push(cnt);
        }
    }
        
    res
}
​