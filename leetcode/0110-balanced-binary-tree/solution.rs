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

type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_balanced(root: Link) -> bool {
        fn dfs(node: Link) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node_ref = node.as_ref().unwrap().borrow();
            
            let left = node_ref.left.clone();
            let left_h = dfs(left);
            if left_h == -1 {
                return -1;
            }
            
            let right = node_ref.right.clone();
            let right_h = dfs(right);
            if right_h == -1 {
                return -1;
            }

            if (left_h - right_h).abs() > 1 {
                return -1;
            }

            1 + left_h.max(right_h)
        }

        dfs(root) != -1
    }
}
