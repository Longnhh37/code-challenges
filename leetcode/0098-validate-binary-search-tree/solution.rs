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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root, None, None)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, low: Option<i64>, high: Option<i64>) -> bool {
        let Some(node) = root else { return true };
        let n = node.borrow();
        let val = n.val as i64;

        if low.is_some_and(|l| val <= l) || high.is_some_and(|h| val >= h) {
            return false;
        }
        Self::dfs(n.left.clone(), low, Some(val)) && Self::dfs(n.right.clone(), Some(val), high)
    }
}
