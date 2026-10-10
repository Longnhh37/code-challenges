use std::io::{self, Read};

const DELTA: [(isize, isize); 8] = [
    (1, 2),
    (2, 1),
    (-1, 2),
    (-2, 1),
    (1, -2),
    (2, -1),
    (-1, -2),
    (-2, -1),
];

enum Outcome {
    Win(u32),
    Stalemate(u32),
    Loss(u32),
}

fn solve(rows: usize, cols: usize, mut pr: usize, pc: usize, kr: usize, kc: usize) -> Outcome {
    if pr == rows {
        return Outcome::Loss(0);
    }
    if kr == pr + 1 && kc == pc {
        return Outcome::Stalemate(0);
    }

    let mut cur = vec![vec![false; cols + 1]; rows + 1];
    cur[kr][kc] = true;
    let mut stalemate = None;

    for n in 1u32.. {
        pr += 1;

        if pr == rows {
            return match stalemate {
                Some(s) => Outcome::Stalemate(s),
                None => Outcome::Loss(n - 1),
            };
        }

        let mut nxt = vec![vec![false; cols + 1]; rows + 1];

        for r in 1..=rows {
            for c in 1..=cols {
                if !cur[r][c] {
                    continue;
                }
                for (dr, dc) in DELTA {
                    let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
                    else {
                        continue;
                    };
                    if (1..=rows).contains(&nr) && (1..=cols).contains(&nc) {
                        nxt[nr][nc] = true;
                    }
                }
            }
        }

        if nxt[pr][pc] {
            return Outcome::Win(n);
        }
        if nxt[pr + 1][pc] {
            stalemate.get_or_insert(n);
            nxt[pr + 1][pc] = false;
        }

        cur = nxt;
    }

    unreachable!()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = it.next().unwrap();
    for _ in 0..n {
        let rows = it.next().unwrap();
        let cols = it.next().unwrap();
        let pr = it.next().unwrap();
        let pc = it.next().unwrap();
        let kr = it.next().unwrap();
        let kc = it.next().unwrap();
        match solve(rows, cols, pr, pc, kr, kc) {
            Outcome::Win(m) => println!("Win in {m} knight move(s)."),
            Outcome::Stalemate(m) => println!("Stalemate in {m} knight move(s)."),
            Outcome::Loss(m) => println!("Loss in {m} knight move(s)."),
        }
    }
}
