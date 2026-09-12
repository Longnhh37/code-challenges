use std::rc::Rc;
use std::cell::RefCell;

type Link = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn max_path_sum(root: Option<Link>) -> i32 {
        let mut max_sum = i32::MIN;
        Self::dfs(&root, &mut max_sum);
        max_sum
    }
    
    fn dfs(root: &Option<Link>, max_sum: &mut i32) -> i32 {
        let Some(node) = root else { return 0 };
        let n = node.borrow();
        let cur = n.val;
        let left = Self::dfs(&n.left, max_sum).max(0);
        let right = Self::dfs(&n.right, max_sum).max(0);

        *max_sum = (*max_sum).max(cur + left + right);
        cur + left.max(right)
    }
}
