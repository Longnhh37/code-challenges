struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}
 
impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
 
    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }
 
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut ra, mut rb) = (self.find(a), self.find(b));
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 1 {
            return true;
        }
        if nums.contains(&1) {
            return false;
        }

        let max = *nums.iter().max().unwrap() as usize;

        let mut spf: Vec<usize> = (0..=max).collect();
        let mut i = 2;
        while i * i <= max {
            if spf[i] == i {
                for j in (i * i..=max).step_by(i) {
                    if spf[j] == j {
                        spf[j] = i;
                    }
                }
            }
            i += 1;
        }

        // Nodes [0, n) are indices. Nodes n + p is the prime of p
        let mut dsu = Dsu::new(n + max + 1);
        for (idx, &num) in nums.iter().enumerate() {
            let mut x = num as usize;
            while x > 1 {
                let p = spf[x];
                dsu.union(idx, n + p);
                while x % p == 0 {
                    x /= p;
                }
            }
        }

        let root = dsu.find(0);
        (1..n).all(|i| dsu.find(i) == root)
    }
}
