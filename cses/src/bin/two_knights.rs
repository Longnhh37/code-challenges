use std::io::Read;

const DIRS: [(i16, i16); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (1, -2),
    (2, 1),
    (2, -1),
    (1, 2),
    (1, -2),
];

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: i16 = input.trim().parse().unwrap();

    let mut res = 0;

    for i in 0..n {
        for j in 0..n {
            let mut see = 0;
            for (di, dj) in DIRS {
                let (ni, nj) = (i + di, j + dj);
                if 0 <= ni && ni < n && 0 <= nj && nj <= n {
                    see += 1;
                }
            }
            res += n * n - see;
        }
    }

    println!("{res}");
}
