use std::rc::Rc;
use std::cell::RefCell;

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
//             right: None
//         }
//     }
// }

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut total = 0;
        Self::dfs(&root, low, high, &mut total);
        total
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, total: &mut i32) {
        let Some(node) = root else { return; };
        let n = node.borrow();
        if n.val < low {
            Self::dfs(&n.right, low, high, total);
        } else if n.val > high {
            Self::dfs(&n.left, low, high, total);
        } else {
            *total += n.val;
            Self::dfs(&n.left, low, high, total);
            Self::dfs(&n.right, low, high, total);
        }
    }
}
