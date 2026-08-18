fn find_even_index(arr: &[i32]) -> Option<usize> {
    let mut pref = vec![0; arr.len() + 1];
    
    for i in 1..arr.len() + 1 {
        pref[i] = pref[i - 1] + arr[i - 1];
    }
    
    let last = pref[pref.len() - 1];
    for i in 1..pref.len() {
        if pref[i - 1] == last - pref[i] {
            return Some(i - 1);
        }
    }
    
    None
}
​