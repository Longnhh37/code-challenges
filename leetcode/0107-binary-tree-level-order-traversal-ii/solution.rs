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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
       let mut levels = Vec::new();

       let Some(node) = root else { return levels };
       let mut q = VecDeque::new();
       q.push_back(node);

       while !q.is_empty() {
            let mut cur = Vec::new();
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                cur.push(n.val);
                if n.left.is_some() {
                    q.push_back(n.left.clone().unwrap());
                }
                if n.right.is_some() {
                    q.push_back(n.right.clone().unwrap());
                }
            }
            levels.push(cur);
        }

        levels.reverse();
        levels
    }
}
