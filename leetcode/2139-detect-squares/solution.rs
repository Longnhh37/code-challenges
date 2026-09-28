use std::collections::HashMap;

struct DetectSquares {
    freq: HashMap<(i32, i32), u32>,
    point: HashMap<i32, HashMap<i32, u32>>,
}

impl DetectSquares {

    fn new() -> Self {
        Self {
            freq: HashMap::new(),
            point: HashMap::new(),
        }
    }
    
    fn add(&mut self, p: Vec<i32>) {
        let (x, y) = (p[0], p[1]);
        *self.freq.entry((x, y)).or_insert(0) += 1;
        *self.point.entry(x).or_default().entry(y).or_default() += 1;
    }
    
    fn count(&self, query: Vec<i32>) -> i32 {
        let mut res = 0;
        let (qx, qy) = (query[0], query[1]);

        if let Some(inner_map) = self.point.get(&qx) {
            for (&y, &c1) in inner_map {
                if qy - y == 0 {
                    continue;
                }
                let d = (qy - y).abs();
                if let Some(&c2) = self.freq.get(&(qx + d, qy)) 
                    && let Some(&c3) = self.freq.get(&(qx + d, y)) {
                        res += c1 * c2 * c3;
                    }

                if let Some(&c2) = self.freq.get(&(qx - d, qy)) 
                    && let Some(&c3) = self.freq.get(&(qx - d, y)) {
                        res += c1 * c2 * c3;
                    }
            }
        }

        res as i32
    }
}
