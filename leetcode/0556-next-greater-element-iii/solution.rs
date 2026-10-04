impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut d = Self::to_digits(n);
        let n = d.len();

        let Some(i) = (0..n - 1).rev().find(|&i| d[i] < d[i + 1]) else {
            return -1;
        };
        let j = (i + 1..n).rev().find(|&j| d[j] > d[i]).unwrap();
        d.swap(i, j);
        d[i + 1..].reverse();

        let res = Self::to_u64(&d);
        match i32::try_from(res) {
            Ok(v) => v,
            Err(_) => -1,
        }
    }

    fn to_digits(mut n: i32) -> Vec<u8> {
        let mut res = Vec::new();
        while n > 0 {
            res.push((n % 10) as u8);
            n /= 10;
        }
        res.reverse();
        res
    }

    fn to_u64(arr: &[u8]) -> u64 {
        let mut res = 0;
        for &n in arr {
            res = res * 10 + n as u64;
        }
        res
    }
}
