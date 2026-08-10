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

type Link = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn merge_trees(root1: Option<Link>, root2: Option<Link>) -> Option<Link> {
        match (root1, root2) {
            (None, r) => r,
            (l, None) => l,
            (Some(n1), Some(n2)) => {
                n1.borrow_mut().val += n2.borrow().val;
                let left = Self::merge_trees(
                    n1.borrow().left.clone(), 
                    n2.borrow().left.clone()
                );
                let right = Self::merge_trees(
                    n1.borrow().right.clone(),
                    n2.borrow().right.clone()
                );
                n1.borrow_mut().left = left;
                n1.borrow_mut().right = right;
                Some(n1)
            }
        }
    }
}
