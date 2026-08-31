fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let (mut min1, mut min2) = (u32::MAX, u32::MAX);
    
    for &n in numbers {
        if n <= min1 {
            (min1, min2) = (n, min1);
        } else if n < min2 {
            min2 = n;
        }
    }
    
    min1 + min2
}