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

type TreeLink = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_valid_bst(root: Option<TreeLink>) -> bool {
        Self::is_within_bound(root, i32::MIN, i32::MAX)
    }

    fn is_within_bound(node: Option<TreeLink>, lower: i32, upper: i32) -> bool {
        let Some(node) = node else { return true };
        let node_ref = node.borrow();
        let val = node_ref.val;
        if !(lower < val && val < upper) {
            return false;
        }
        if !Self::is_within_bound(node_ref.left.clone(), lower, val) {
            return false;
        }
        Self::is_within_bound(node_ref.right.clone(), val, upper)
    }
}
