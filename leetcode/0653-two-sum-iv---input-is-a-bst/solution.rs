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
use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let Some(node) = root else { return false; };
        let mut q = VecDeque::from([node.clone()]);
        let mut visited = HashSet::new();
        while let Some(node) = q.pop_front() {
            let n = node.borrow();
            if visited.contains(&(k - n.val)) {
                return true;
            }
            visited.insert(n.val);
            if let Some(left) = &n.left {
                q.push_back(left.clone());
            }
            if let Some(right) = &n.right {
                q.push_back(right.clone());
            }
        }
        false
    }
}
