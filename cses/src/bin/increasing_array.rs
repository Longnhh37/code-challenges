use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let n = it.next().unwrap() as usize;

    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(it.next().unwrap());
    }

    let mut res = 0;
    for i in 1..n {
        if arr[i] < arr[i - 1] {
            res += arr[i - 1] - arr[i];
            arr[i] = arr[i - 1];
        }
    }

    println!("{res}");
}
