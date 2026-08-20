pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;
    let len = n * 2 + 1;

    let mut out = Vec::with_capacity(len);

    for i in 0..=n {
        let ch = (b'A' + i as u8) as char;
        let left = n - i;
        let right = n + i;

        let mut row = String::with_capacity(len);

        for j in 0..len {
            row.push(if j == left || j == right { ch } else { '.' });
        }

        out.push(row);
    }

    for i in (0..n).rev() {
        out.push(out[i].clone());
    }
    out
}

fn main() {
    dbg!(get_diamond('G'));
}
