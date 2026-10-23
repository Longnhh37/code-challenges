impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let (n, k) = (n as usize, k as usize);
        let nodes = k.pow(n as u32 - 1);
        let mut next = vec![0usize; nodes];

        let mut stack: Vec<(usize, u8)> = vec![(0, 0)];
        let mut edges: Vec<u8> = Vec::with_capacity(k.pow(n as u32) + 1);

        while let Some(&(v, d)) = stack.last() {
            if next[v] < k {
                let e = next[v];
                next[v] += 1;
                stack.push(((v * k + e) % nodes, e as u8));
            } else {
                edges.push(d);
                stack.pop();
            }
        }

        edges.pop();
        edges.reverse();

        let mut res = String::with_capacity(edges.len() + n - 1);
        res.extend(std::iter::repeat_n('0', n - 1));
        res.extend(edges.iter().map(|&d| (b'0' + d) as char));
        res
    }
}
