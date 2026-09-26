use std::io::Read;

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let grid: Vec<&[u8]> = it.map(str::as_bytes).collect();
    let m = grid[0].len();

    if grid[0][0] == b'*' || grid[n - 1][m - 1] == b'*' {
        println!("0");
        return;
    }

    let mut prev = vec![0u64; m];
    prev[0] = 1;
    for j in 1..m {
        prev[j] = if grid[0][j] == b'*' { 0 } else { prev[j - 1] };
    }

    for i in 1..n {
        let mut cur = vec![0u64; m];
        cur[0] = if grid[i][0] == b'*' { 0 } else { prev[0] };
        for j in 1..m {
            cur[j] = if grid[i][j] == b'*' {
                0
            } else {
                (cur[j - 1] + prev[j]) % MOD
            }
        }
        prev = cur;
    }

    println!("{}", prev[m - 1]);
}
