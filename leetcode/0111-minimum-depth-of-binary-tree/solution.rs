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
use std::collections::VecDeque;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(node) = root else { return 0; };
        let mut q = VecDeque::from([node]);
        let mut level = 0;

        while !q.is_empty() {
            level += 1;
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                if n.left.is_none() && n.right.is_none() {
                    return level;
                }
                if let Some(left) = &n.left {
                    q.push_back(left.clone());
                }
                if let Some(right) = &n.right {
                    q.push_back(right.clone());
                }
            }
        }
        
        level
    }
}
