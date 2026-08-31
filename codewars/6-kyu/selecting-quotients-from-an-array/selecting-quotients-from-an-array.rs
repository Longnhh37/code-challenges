fn select_quotients(arr: &[u32], m: u32, dir_str: &str) -> Vec<(u32, (u32, u32))> {
    let mut arr = arr.to_vec();
    arr.sort_unstable();
    arr.dedup();
    let mut res = Vec::new();
    
    for i in 0..arr.len() - 1 {
        for j in i + 1..arr.len() {
            if arr[j] % arr[i] == 0 {
                let div = arr[j] / arr[i];
                if div < m 
                || (dir_str.to_ascii_lowercase() == "odd" && div % 2 != 1)
                || (dir_str.to_ascii_lowercase() == "even" && div % 2 != 0)
                {
                    continue;
                } 
                res.push((div, (arr[j], arr[i])));
            }
        }
    }
    res.sort_unstable();
    res
}