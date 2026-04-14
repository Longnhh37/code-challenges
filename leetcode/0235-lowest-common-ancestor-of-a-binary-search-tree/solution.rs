// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let left = p.as_ref().unwrap().borrow().val.clone();
        let right = q.as_ref().unwrap().borrow().val.clone();
        let (left, right) = (left.min(right), left.max(right));

        let mut cur = root.clone();
        while let Some(node) = cur {
            let val = node.borrow().val;
            if left > val {
                cur = node.borrow().right.clone();
            } else if right < val {
                cur = node.borrow().left.clone();
            } else {
                return Some(node);
            }
        }

    None
    }
}
