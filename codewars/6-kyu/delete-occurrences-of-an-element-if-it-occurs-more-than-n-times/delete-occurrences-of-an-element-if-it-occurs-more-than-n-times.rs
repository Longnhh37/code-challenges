use std::collections::HashMap;
​
fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counter = HashMap::new();   
    let mut res = Vec::new();
    
    for &cur in lst {
        let cnt = counter.entry(cur).or_insert(0usize);
        if *cnt < n {
            res.push(cur);
            *cnt += 1;
        }
    }
    
    res
}
​