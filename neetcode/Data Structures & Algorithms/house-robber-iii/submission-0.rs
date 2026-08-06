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
    pub fn rob(root: Option<Link>) -> i32 {
        fn dfs(node: &Option<Link>) -> (i32, i32) {
            let Some(n) = node else { return (0, 0) };
            let n = n.borrow();
            let left_pair = dfs(&n.left);
            let right_pair = dfs(&n.right);
            let with_root = n.val + left_pair.1 + right_pair.1;
            let without_root = left_pair.0.max(left_pair.1) + right_pair.0.max(right_pair.1);
            (with_root, without_root)
        }

        let res = dfs(&root);
        res.0.max(res.1)
    }
}
