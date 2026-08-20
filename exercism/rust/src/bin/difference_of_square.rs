
pub fn square_of_sum(n: u64) -> u64 {
    (1..=n).sum::<u64>().pow(2)
}

pub fn sum_of_squares(n: u64) -> u64 {
    (1..=n)
        .map(|x| x * x)
        .sum::<u64>()
}

pub fn difference(n: u64) -> u64 {
    square_of_sum(n) - sum_of_squares(n)
}

fn main() {
    let x = difference(10);
    println!("{x}");
}
