pub fn solution(array_a: &[i64], array_b: &[i64]) -> f64 {
    let len = array_a.len();
    let mut sum = 0;
    
    for (a, b) in array_a.iter().zip(array_b.iter()) {
        sum += (a.max(b) - a.min(b)).pow(2);
    }
    
    sum as f64 / len as f64
}
​