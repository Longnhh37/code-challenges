use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let n = it.next().unwrap();

    let mut sum = 0;
    for n in it {
        sum += n;
    }

    let missing = (n + 1) * n / 2 - sum;
    println!("{missing}");
}
