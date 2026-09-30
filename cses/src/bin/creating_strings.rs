use std::fmt::Write;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut bytes: Vec<u8> = input.trim().bytes().collect();
    bytes.sort_unstable();
    let n = bytes.len();

    let mut res = Vec::new();
    let mut visited = vec![false; n];
    permute(&bytes, &mut visited, &mut Vec::new(), &mut res);

    let mut out = String::with_capacity(res.len() * n * 2);
    writeln!(out, "{}", res.len()).unwrap();

    for bytes in res {
        writeln!(out, "{}", String::from_utf8(bytes).unwrap()).unwrap();
    }

    println!("{}", out);
}

fn permute(bytes: &[u8], visited: &mut [bool], path: &mut Vec<u8>, res: &mut Vec<Vec<u8>>) {
    if path.len() == bytes.len() {
        res.push(path.clone());
        return;
    }

    for i in 0..bytes.len() {
        if visited[i] {
            continue;
        }
        let b = bytes[i];
        if i > 0 && b == bytes[i - 1] && !visited[i - 1] {
            continue;
        }

        visited[i] = true;
        path.push(b);
        permute(bytes, visited, path, res);
        path.pop();
        visited[i] = false;
    }
}
