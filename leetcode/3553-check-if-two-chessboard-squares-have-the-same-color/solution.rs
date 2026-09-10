impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let b1 = coordinate1.as_bytes();
        let (c1, r1) = (b1[0], b1[1]);

        let b2 = coordinate2.as_bytes();
        let (c2, r2) = (b2[0], b2[1]);

        let (c1, c2) = (c1 -  b'a', c2 - b'a');
        let (r1, r2) = (r1 - b'1', r2 - b'1');

        (c1 & 1 == c2 & 1 && r1 & 1 == r2 & 1)
        || (c1 & 1 == r1 & 1 && c2 & 1 == r2 & 1)
        || (c1 & 1 == r2 & 1 && c2 & 1 == r1 & 1)
    }
}
