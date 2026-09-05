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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root, &root)
    }

    fn dfs(r1: &Option<Rc<RefCell<TreeNode>>>, r2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (&r1, &r2) {
            (Some(n1), Some(n2)) => {
                n1.borrow().val == n2.borrow().val 
                && Self::dfs(&n1.borrow().left, &n2.borrow().right)
                && Self::dfs(&n1.borrow().right, &n2.borrow().left)
            }
            (None, None) => true,
            _ => false,
        }
    }
}
