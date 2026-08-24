mod solution {
    
    pub fn range_extraction(a: &[i32]) -> String {
        if a.is_empty() {
            return String::new();
        }
        
        let mut arr = a.to_vec();
        arr.sort_unstable();
        let mut res = String::new();
        
        let mut l = 0;
        for r in 1..arr.len() {
            if arr[r] != arr[r - 1] + 1 {
                if r - l < 3 {
                    while l < r {
                        res.push_str(&arr[l].to_string());
                        res.push(',');
                        l += 1;
                    }
                } else {
                    res.push_str(&arr[l].to_string());
                    res.push('-');
                    res.push_str(&arr[r - 1].to_string());
                    res.push(',');
                    l = r;
                }
            }
        }
        
        let len = arr.len();
        if l == len - 2 {
            res.push_str(&arr[len - 2].to_string());
            res.push(',');
            res.push_str(&arr[len - 1].to_string());
        } else if l == len - 1 {
            res.push_str(&arr[len - 1].to_string());
        } else {
            res.push_str(&arr[l].to_string());
            res.push('-');
            res.push_str(&arr[len - 1].to_string());
        }
        
        res
    }
}