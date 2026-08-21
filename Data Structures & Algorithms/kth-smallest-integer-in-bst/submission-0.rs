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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        Self::dfs(&root, &mut k).unwrap()
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> Option<i32> {
        let Some(n) = node else {
            return None;
        };
        let n = n.borrow();

        if let Some(ans) = Self::dfs(&n.left, k) {
            return Some(ans);
        }

        *k -= 1;
        if *k == 0 {
            return Some(n.val);
        }

        Self::dfs(&n.right, k)
    }
}
