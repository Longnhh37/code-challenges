use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let n = it.next().unwrap();

    let mut tasks: Vec<_> = (0..n)
        .map(|_| {
            let duration = it.next().unwrap();
            let dl = it.next().unwrap();
            (duration, dl)
        })
        .collect();
    tasks.sort_unstable();

    let mut reward = 0;
    let mut time = 0;

    for (dur, dl) in tasks {
        time += dur;
        reward += dl - time;
    }

    println!("{}", reward);
}
