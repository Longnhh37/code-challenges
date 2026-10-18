use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let n = it.next().unwrap() as usize;
    let a: Vec<_> = it.take(n).collect();

    let left = {
        let mut res = vec![-1i64; n];
        let mut stack = Vec::new();
        for i in 0..n {
            while let Some(&top) = stack.last() {
                if a[top] >= a[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            res[i] = stack.last().map_or(-1, |&idx| idx as i64);
            stack.push(i);
        }
        res
    };

    let right = {
        let mut res = vec![-1i64; n];
        let mut stack = Vec::new();
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if a[top] >= a[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            res[i] = stack.last().map_or(n as i64, |&idx| idx as i64);
            stack.push(i);
        }
        res
    };

    let out = (0..n)
        .map(|i| a[i] * (right[i] - left[i] - 1))
        .max()
        .unwrap_or(0);
    println!("{out}");
}
