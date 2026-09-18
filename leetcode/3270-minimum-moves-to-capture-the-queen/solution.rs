impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        let between = |x: i32, lo: i32, hi: i32| -> bool {
            (lo < x && x < hi) || (hi < x && x < lo)
        };

        let rook_row_clear = a == e && !(c == a && between(d, b, f));
        let rook_col_clear = b == f && !(d == b && between(c, a, e));

        let bishop_main_clear = (c - d == e - f) && 
            !(a - b == e - f && between(a, c, e));
        let bishop_anti_clear = (c + d == e + f) &&
            !(a + b == e + f && between(a, c, e));
        
        if rook_row_clear || rook_col_clear || bishop_main_clear || bishop_anti_clear {
            1
        } else {
            2
        }
    }   
}
