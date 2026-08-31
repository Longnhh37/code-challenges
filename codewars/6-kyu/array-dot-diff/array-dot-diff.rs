use std::collections::HashSet;
​
fn array_diff<T: Eq + std::hash::Hash + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let set_a: HashSet<T> = a.iter().cloned().collect();
    let set_b: HashSet<T> = b.iter().cloned().collect();
    
    let diff: HashSet<_> = set_a.difference(&set_b).collect();
    
    a
        .iter()
        .filter(|&n| diff.contains(n))
        .cloned()
        .collect()
}