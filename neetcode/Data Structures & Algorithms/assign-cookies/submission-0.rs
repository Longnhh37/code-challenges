impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();

        let mut res = 0;
        let mut j = 0;
        for i in 0..s.len() {
            if g[j] <= s[i] {
                res += 1;
                j += 1;
                if j >= g.len() {
                    break;
                }
            }
        }

        res
    }
}
