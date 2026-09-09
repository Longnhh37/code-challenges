use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root, 0)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        let Some(node) = root else { return depth };
        let n = node.borrow();
        max(
            Self::dfs(n.left.clone(), depth + 1),
            Self::dfs(n.right.clone(), depth + 1),
        )
    }
}
