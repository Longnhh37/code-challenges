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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let Some(node) = root else { return Vec::new(); };
        let mut q = VecDeque::new();       
        q.push_back(node);
        let mut res = Vec::new();

        while !q.is_empty() {
            let size = q.len();
            for i in 0..size {
                let node = q.pop_front().unwrap();
                let n = node.borrow();

                if i == size - 1 {
                    res.push(n.val);
                }
                if n.left.is_some() {
                    q.push_back(n.left.clone().unwrap());
                }
                if n.right.is_some() {
                    q.push_back(n.right.clone().unwrap());
                }
            }
        }

        res
    }
}
