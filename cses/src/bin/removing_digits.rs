use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut n: u32 = input.trim().parse().unwrap();
    let mut cnt = 0;
    let mut digits = [false; 10];

    while n > 0 {
        cnt += 1;
        n -= find_biggest_digits(n, &mut digits)
    }
    println!("{}", cnt);
}

fn find_biggest_digits(mut n: u32, digits: &mut [bool; 10]) -> u32 {
    while n > 0 {
        digits[(n % 10) as usize] = true;
        n /= 10;
    }
    let n = digits.iter().rposition(|&b| b).unwrap();
    reset_digits(digits);
    n as u32
}

fn reset_digits(digits: &mut [bool; 10]) {
    for b in digits.iter_mut() {
        *b = false;
    }
}
