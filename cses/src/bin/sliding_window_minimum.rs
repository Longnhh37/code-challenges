use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<u32>().unwrap());
    let n = it.next().unwrap();
    let size = it.next().unwrap();

    let x = it.next().unwrap();
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    let c = it.next().unwrap();


}
