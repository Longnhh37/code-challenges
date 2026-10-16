use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_ascii_whitespace();
    let mut get_next_param = || -> usize {
        let next = it.next().unwrap().split_once('=').unwrap().1;
        let n: usize = next[..next.len() - 1].parse().unwrap();
        n
    };

    let rows = get_next_param();
    let cols = get_next_param();
    let height = get_next_param();
    let width = get_next_param();

    it.next();

    let arr: Vec<Vec<u32>> = (0..rows)
        .map(|_| {
            it.by_ref()
                .take(cols)
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("{}", min_median(&arr, height, width));
}

fn min_median(arr: &[Vec<u32>], height: usize, width: usize) -> u32 {
    let (rows, cols) = (arr.len(), arr[0].len());
    let need = (height * width / 2 + 1) as u32;

    let mut lo = arr.iter().flatten().copied().min().unwrap();
    let mut hi = arr.iter().flatten().copied().max().unwrap();

    let mut pre = vec![vec![0u32; cols + 1]; rows + 1];

    let mut feasible = |x: u32| -> bool {
        for r in 0..rows {
            for c in 0..cols {
                pre[r + 1][c + 1] =
                    (arr[r][c] <= x) as u32 + pre[r][c + 1] + pre[r + 1][c] - pre[r][c];
            }
        }

        for r in height..=rows {
            for c in width..=cols {
                let cnt =
                    pre[r][c] + pre[r - height][c - width] - pre[r - height][c] - pre[r][c - width];
                if cnt >= need {
                    return true;
                }
            }
        }

        false
    };

    while lo < hi {
        let mid = lo.midpoint(hi);
        if feasible(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    lo
}
