const DIRS: [(i32, i32); 9] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 0), (0, 1),
    (1, -1), (1, 0), (1, 1),
];

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (img.len(), img[0].len());
        let (irows, icols) = (rows as i32, cols as i32);
        let mut res = vec![vec![0; cols]; rows];

        for r in 0..rows {
            for c in 0..cols {
                Self::apply_filter(&img, irows, icols, r, c, &mut res);
            }
        } 

        res
    }

    fn apply_filter(image: &[Vec<i32>], irows: i32, icols: i32, r: usize, c: usize, res: &mut Vec<Vec<i32>>) {
        let mut total = 0;
        let mut count = 0;
        let (ir, ic) = (r as i32, c as i32);

        for (dr, dc) in DIRS {
            let (nr, nc) = (ir + dr, ic + dc);
            if 0 <= nr && nr < irows && 0 <= nc && nc < icols {
                count += 1;
                let (ur, uc) = (nr as usize, nc as usize);
                total += image[ur][uc];
            }
        }
        res[r][c] = total / count;
    }
}
