use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut cur: i64, target: i64, freq: &mut HashMap<i64, i32>) -> i32 {
            let Some(n) = node else { return 0 };
            let n = n.borrow();
            cur += n.val as i64;

            let mut count = freq.get(&(cur - target)).copied().unwrap_or(0);

            *freq.entry(cur).or_default() += 1;
            count += dfs(&n.left, cur, target, freq);
            count += dfs(&n.right, cur, target, freq);
            *freq.get_mut(&cur).unwrap() -= 1;

            count
        }

        let mut freq = HashMap::from([(0, 1)]);
        dfs(&root, 0, target_sum as i64, &mut freq)
    }
        
}
