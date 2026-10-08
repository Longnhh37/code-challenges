use std::io::{self, Read};

const ARRIVE: u8 = 0;
const LEAVE: u8 = 1;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();

    let mut customers = Vec::with_capacity(n * 2);
    for _ in 0..n {
        let arrive = it.next().unwrap();
        let leave = it.next().unwrap();
        customers.push((arrive, ARRIVE));
        customers.push((leave, LEAVE));
    }
    customers.sort_unstable();

    let mut most = 0;
    let mut cur = 0;

    for (_, ty) in customers {
        match ty {
            ARRIVE => cur += 1,
            LEAVE => cur -= 1,
            _ => unreachable!(),
        }
        most = most.max(cur);
    }

    println!("{most}");
}
