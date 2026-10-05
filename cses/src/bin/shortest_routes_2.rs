use std::io::{self, Read, Write};

const INF: usize = usize::MAX / 4;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();
    let q = it.next().unwrap();

    let mut d = vec![vec![INF; n]; n];
    for (i, row) in d.iter_mut().enumerate() {
        row[i] = 0;
    }

    for _ in 0..m {
        let a = it.next().unwrap() - 1;
        let b = it.next().unwrap() - 1;
        let c = it.next().unwrap();
        if c < d[a][b] {
            d[a][b] = c;
            d[b][a] = c;
        }
    }

    for k in 0..n {
        let row_k = d[k].clone();
        for row in d.iter_mut() {
            let dik = row[k];
            if dik == INF {
                continue;
            }
            for (dij, &dkj) in row.iter_mut().zip(&row_k) {
                let cand = dik + dkj;
                if cand < *dij {
                    *dij = cand;
                }
            }
        }
    }

    let mut out = String::with_capacity(q * 8);
    for _ in 0..q {
        let a = it.next().unwrap() - 1;
        let b = it.next().unwrap() - 1;
        let r = d[a][b];
        if r >= INF {
            out.push_str("-1\n");
        } else {
            out.push_str(&r.to_string());
            out.push('\n');
        }
    }

    io::stdout().write_all(out.as_bytes()).unwrap();
}
