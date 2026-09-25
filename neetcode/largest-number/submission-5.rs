struct CustomString(String);

impl Eq for CustomString {}
impl PartialEq for CustomString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for CustomString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CustomString {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 == other.0 {
            return Ordering::Equal;
        }

        let mut s1: Vec<char> = self.0.chars().collect();
        s1.extend(other.0.chars());
        let mut s2: Vec<char> = other.0.chars().collect();
        s2.extend(self.0.chars());
        
        let mut i = 0;
        while i < s1.len() {
            if s1[i] > s2[i] {
                return Ordering::Greater;
            } else if s1[i] < s2[i] {
                return Ordering::Less;
            } else {
                i += 1;
            }
        }
        Ordering::Equal
    }
}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.iter().all(|&n| n == 0) {
            return "0".to_string();
        }

        let mut nums: Vec<CustomString> = nums.into_iter()
            .map(|n| CustomString(n.to_string()))
            .collect();
        
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut res = String::new();
        for n in &nums {
            res.push_str(&n.0);
        }
        
        res
    }
}
