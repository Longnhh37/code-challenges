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
    pub fn lowest_common_ancestor(
        root: Option<TreeLink>,
        p: Option<TreeLink>,
        q: Option<TreeLink>,
    ) -> Option<TreeLink> {
        let (p, q) = match (p, q) {
            (None, None) => return root,
            (Some(p), None) => return Some(p),
            (None, Some(q)) => return Some(q),
            (Some(p), Some(q)) => (p, q)
        };

        let p_val = p.borrow().val;
        let q_val = q.borrow().val;

        let mut cur = root;
        while let Some(node) = cur {
            let node_val = node.borrow().val;

            if p_val < node_val && q_val < node_val {
                cur = node.borrow().left.clone();
            } else if  p_val > node_val && q_val > node_val {
                cur = node.borrow().right.clone()
            } else {
                return Some(node);
            }
        }
        None
    }
}
