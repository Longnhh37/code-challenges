impl Solution {
    pub fn flood_fill(
        mut image: Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        color: i32,
    ) -> Vec<Vec<i32>> {
        let m = image.len();
        let n = image[0].len();

        let sr = sr as usize;
        let sc = sc as usize;

        let start_value = image[sr][sc];

        if start_value == color {
            return image;
        }

        fn dfs(
            image: &mut Vec<Vec<i32>>,
            r: i32,
            c: i32,
            start_value: i32,
            color: i32,
            m: i32,
            n: i32,
        ) {
            if r < 0 || c < 0 || r >= m || c >= n {
                return;
            }

            let (r_usize, c_usize) = (r as usize, c as usize);


            if image[r_usize][c_usize] != start_value {
                return;
            }

            image[r_usize][c_usize] = color;

            dfs(image, r - 1, c, start_value, color, m, n);
            dfs(image, r + 1, c, start_value, color, m, n);
            dfs(image, r, c - 1, start_value, color, m, n);
            dfs(image, r, c + 1, start_value, color, m, n);
        }

        dfs(
            &mut image,
            sr as i32,
            sc as i32,
            start_value,
            color,
            m as i32,
            n as i32,
        );

        image
    }
}

