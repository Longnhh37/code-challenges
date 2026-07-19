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
    pub fn remove_leaf_nodes(mut root: Option<Link>, target: i32) -> Option<Link> {
        Self::dfs(&mut root, target);
        root
    }

    fn dfs(root: &mut Option<Link>, target: i32) {
        let Some(node) = root.as_ref() else { return };

        Self::dfs(&mut node.borrow_mut().left, target);
        Self::dfs(&mut node.borrow_mut().right, target);

        let is_leaf = node.borrow().left.is_none() && node.borrow().right.is_none();
        let val = node.borrow().val;

        if val == target && is_leaf {
            root.take();
        }
    }
}
