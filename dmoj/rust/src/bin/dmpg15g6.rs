use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

struct XorShift(u64);

impl XorShift {
    fn new() -> Self {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut z = t.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        Self((z ^ (z >> 31)) | 1)
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

const K: usize = 60;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input
        .split_ascii_whitespace()
        .map(|t| t.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let a: Vec<usize> = it.by_ref().take(n).collect();

    let mut pos: Vec<Vec<u32>> = vec![Vec::new(); m + 1];
    for (i, &f) in a.iter().enumerate() {
        pos[f].push(i as u32);
    }

    let count = |f: usize, l: usize, r: usize| -> usize {
        let p = &pos[f];
        p.partition_point(|&x| x as usize <= r) - p.partition_point(|&x| (x as usize) < l)
    };

    let q = it.next().unwrap();
    let mut rng = XorShift::new();
    let mut out = String::new();

    for _ in 0..q {
        let l = it.next().unwrap() - 1;
        let r = it.next().unwrap() - 1;
        let len = r - l + 1;

        let mut good = [0usize; 2];
        let mut bad = 0;
        let mut ok = false;

        for _ in 0..K {
            let f = a[l + (rng.next() % len as u64) as usize];
            if good[..bad].contains(&f) {
                continue;
            }
            let c = count(f, l, r);
            if 3 * c >= 2 * len {
                ok = true;
                break;
            }
            if 3 * c >= len {
                good[bad] = f;
                bad += 1;
                if bad == 2 {
                    ok = true;
                    break;
                }
            }
        }

        out.push_str(if ok { "YES\n" } else { "NO\n" });
    }
    print!("{out}");
}
