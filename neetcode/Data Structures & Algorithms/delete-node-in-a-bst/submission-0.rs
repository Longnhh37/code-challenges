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
    pub fn delete_node(root: Option<Link>, key: i32) -> Option<Link> {
        let Some(node) = root else { return None };
        let node_val = node.borrow().val;

        if key < node_val {
            let left = node.borrow().left.clone();
            node.borrow_mut().left = Self::delete_node(left, key);
            Some(node)
        } else if key > node_val {
            let right = node.borrow().right.clone();
            node.borrow_mut().right = Self::delete_node(right, key);
            Some(node)
        } else  {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            match (left, right) {
                (None, None) => None,
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (Some(_), Some(right_child)) => {
                    let successor_val = Self::find_min(&right_child);
                    node.borrow_mut().val = successor_val;
                    node.borrow_mut().right = Self::delete_node(Some(right_child), successor_val);
                    Some(node)
                }
            }
        }
        
    }

    fn find_min(node: &Link) -> i32 {
        let left = node.borrow().left.clone();
        match left {
            None => node.borrow().val,
            Some(l) => Self::find_min(&l),
        }
    }
}
