fn expanded_form(mut n: u64) -> String {
    let mut unit = 1;
    let mut res = Vec::new();
    
    while n > 0 {
        let d = n % 10;
        
        if d != 0 {
            res.push(d * unit);
        }
        n /= 10;
        unit *= 10;
    }
    
    res.reverse();
    let res: Vec<String> = res.into_iter().map(|n| n.to_string()).collect();
    res.join(" + ")
}