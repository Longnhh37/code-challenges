use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_ascii_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let geese = it.next().unwrap().as_bytes().to_vec();
    let gscore: Vec<usize> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();
    let hawk = it.next().unwrap().as_bytes().to_vec();
    let hscore: Vec<usize> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();

    let mut dp = vec![vec![0usize; n + 1]; n + 1];

    for i in 1..=n {
        let (gr, gp) = (geese[i - 1], gscore[i - 1]);
        for j in 1..=n {
            let (hr, hp) = (hawk[j - 1], hscore[j - 1]);

            dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);

            let rivarlry = match (gr, hr) {
                (b'W', b'L') => gp > hp,
                (b'L', b'W') => gp < hp,
                _ => false,
            };
            if rivarlry {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + gp + hp);
            }
        }
    }

    println!("{}", dp[n][n]);
}
