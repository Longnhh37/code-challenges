use std::collections::HashMap;
​
fn duplicate_encode(word:&str)->String {
    let mut map = HashMap::new();
    
    for c in word.chars() {
        let c = c.to_ascii_lowercase();
        *map.entry(c).or_insert(0) += 1;
    }
    
    let mut res = String::with_capacity(word.len());
    for c in word.chars() {
        let c = c.to_ascii_lowercase();
        if let Some(&1) = map.get(&c) {
            res.push('(');
        } else {
            res.push(')');
        }
    }
    
    res
}