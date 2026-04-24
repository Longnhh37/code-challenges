impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        
        let mut beg = 0;
        let mut out = Vec::new();

        for end in 0..nums.len() - 1 {   
            if nums[end] + 1 != nums[end + 1] {
               if beg == end {
                    out.push(format!("{}", nums[beg]));
               } else {
                    out.push(format!("{}->{}", nums[beg], nums[end]));
               }
                beg = end + 1;
            }
        }

        if beg == nums.len() - 1 {
            out.push(format!("{}", nums[beg]));
        } else {
            out.push(format!("{}->{}", nums[beg], nums.last().unwrap()));
        }

        out
    }
}
