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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::dfs(&root, &mut res);
        res
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left = Self::dfs(&node.left, res);
                let right = Self::dfs(&node.right, res);
                *res = (*res).max(left + right);
                1 + left.max(right)
            }
        }
    }
}
