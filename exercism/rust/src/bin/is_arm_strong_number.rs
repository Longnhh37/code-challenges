pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len() as u32;

    let total: u64 = num_str
        .chars()
        .map(|c| (c.to_digit(10).unwrap() as u64).pow(len))
        .sum();

    total == num as u64
}

fn main() {
    let out = is_armstrong_number(9475);
    println!("{out}");

    let out2 = 9_i32.pow(4) + 2 * 4_i32.pow(4) + 7_i32.pow(4);
    println!("{out2}");
}
