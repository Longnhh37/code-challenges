use std::collections::HashMap;
​
fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    let mut map = HashMap::new();
    for (j, &n) in numbers.iter().enumerate() {
        if let Some(i) = map.get(&(target - n)) {
            return (*i, j);
        } else {
            map.insert(n, j);
        }
    }
    unreachable!()
}