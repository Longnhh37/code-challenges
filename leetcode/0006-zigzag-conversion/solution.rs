impl Solution {
    pub fn convert(s: String, rows: i32) -> String {
        let bytes = s.as_bytes();
        let len = bytes.len();
        
        if rows == 1 || len <= 1 || len as i32 <= rows {
            return s;
        }

        let mut out = String::with_capacity(len);
        let rows = rows as usize;

        for row in 1..=rows {
            let jump = if row == 1 || row == rows {
                [2 * rows - 2, 2 * rows - 2]
            } else {
                [2 * (rows - row), 2 * (row - 1)]
            };
            let mut p = row - 1;
            out.push(bytes[p] as char);

            loop {
                let d = jump[0];
                if p + d >= len {
                    break;
                }
                p += d;
                out.push(bytes[p] as char);

                let d = jump[1];
                if p + d >= len {
                    break;
                }
                p += d;
                out.push(bytes[p] as char);
            }
        }

        out
    }
}

