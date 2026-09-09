use std::collections::BinaryHeap;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut counter = vec![0; 10001];
        for &i in &barcodes {
            counter[i as usize] += 1;
        }

        let mut heap: BinaryHeap<(i32, i32)> = counter
            .into_iter()
            .enumerate()
            .filter(|(_, c)| *c > 0)
            .map(|(i, c)| (c, i as i32))
            .collect();

        let mut res = Vec::new();
        while let Some((cnt, v)) = heap.pop() {
            if let Some(last) = res.last() && *last == v {
                let (cnt2, v2) = heap.pop().unwrap();
                res.push(v2);
                if cnt2 > 1 { 
                    heap.push((cnt2 - 1, v2));
                }
                heap.push((cnt, v));
            } else {
                res.push(v);
                if cnt > 1 {
                    heap.push((cnt - 1, v));
                }
            }
        }
        res
    }
}
