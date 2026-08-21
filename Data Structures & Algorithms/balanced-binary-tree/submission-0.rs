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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::get_height_imbalance(root) != -1
    }

    fn get_height_imbalance(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                let node_ref = node.borrow();
                let left = Self::get_height_imbalance(node_ref.left.clone());
                let right = Self::get_height_imbalance(node_ref.right.clone());

                if left == -1 || right == -1 || (left - right).abs() > 1 {
                    return -1
                }

                1 + left.max(right)
            }
        }
    }
}
