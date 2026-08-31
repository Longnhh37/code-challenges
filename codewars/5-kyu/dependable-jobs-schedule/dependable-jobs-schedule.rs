use std::collections::{HashMap, VecDeque};
​
fn finish_all(prerequisites: &[(u32, u32)]) -> bool {
    if prerequisites.is_empty() {
        return true;
    }
    
    let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, i32> = HashMap::new();
    
    for &(v, u) in prerequisites {
        adj.entry(u).or_insert_with(Vec::new).push(v);
        in_degree.entry(u).or_insert(0);
        *in_degree.entry(v).or_default() += 1;
    }
     
    let mut q = VecDeque::new();
    for (&k, &v) in &in_degree {
        if v == 0 {
            q.push_back(k);
        }
    }
    
    let mut visited = 0;
    while let Some(cur) = q.pop_front() {
        visited += 1;
        if let Some(neighbors) = adj.get(&cur) {
            for &nei in neighbors {
                let deg = in_degree.get_mut(&nei).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    q.push_back(nei);
                }
            }
        }
    }
    
    visited == in_degree.len()
}
​