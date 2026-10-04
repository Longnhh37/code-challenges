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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let Some(node) = root else { return Vec::new(); };
        let mut res = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(node);

        while !q.is_empty() {
            let size = q.len();
            let mut total: i64 = 0;
            for _ in 0..size {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                total += n.val as i64;
                if let Some(left) = &n.left {
                    q.push_back(left.clone());
                }
                if let Some(right) = &n.right {
                    q.push_back(right.clone());
                }
            }
            res.push(total as f64 / size as f64);
        }

        res
    }
}
