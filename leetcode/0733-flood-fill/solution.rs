use std::collections::VecDeque;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let h = image.len() as i32;
        let w = image[0].len() as i32;

        let starting_color = image[sr as usize][sc as usize];
        if starting_color == color {
            return image;
        }

        let mut queue = VecDeque::new();

        image[sr as usize][sc as usize] = color;
        queue.push_back((sc, sr));

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;

                if 0 <= nx
                    && nx < w
                    && 0 <= ny
                    && ny < h
                    && image[ny as usize][nx as usize] == starting_color
                {
                    image[ny as usize][nx as usize] = color;
                    queue.push_back((nx, ny));
                }
            }
        }

        image
    }
}
