// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::dfs(&root, target_sum, 0)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, mut cum_sum: i32) -> bool {
        let Some(node) = root else { return false; };
        let n = node.borrow();
        cum_sum += n.val;

        if n.left.is_none() && n.right.is_none() {
            return cum_sum == target_sum;
        }
        
        Self::dfs(&n.left, target_sum, cum_sum)
        || Self::dfs(&n.right, target_sum, cum_sum)
    }
}
