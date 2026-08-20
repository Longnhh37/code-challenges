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
    pub fn lowest_common_ancestor(root: Option<Link>, p: Option<Link>, q: Option<Link>) -> Option<Link> {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        Self::dfs(&root, p_val, q_val)
    }

    fn dfs(root: &Option<Link>, p: i32, q: i32) -> Option<Link> {
        let Some(node) = root else { return None; };
        let n = node.borrow();
        let val = n.val;
        if val == p || val == q {
            return Some(node.clone());
        }

        let left = Self::dfs(&n.left, p, q);
        let right = Self::dfs(&n.right, p, q);

        if left.is_some() && right.is_some() {
            return Some(node.clone());
        }
        left.or(right)
    }
}
