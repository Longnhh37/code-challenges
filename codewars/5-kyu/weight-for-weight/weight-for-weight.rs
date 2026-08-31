fn order_weight(s: &str) -> String {
    let mut num: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
    
    num.sort_by(|a, b| {
        let sum_a: u32 = a.chars().map(|c| c.to_digit(10).unwrap()).sum();
        let sum_b: u32 = b.chars().map(|c| c.to_digit(10).unwrap()).sum();
        
        sum_a
            .cmp(&sum_b)
            .then_with(|| a.cmp(b))
    });
    
    num.join(" ")
}
​
​