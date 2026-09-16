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
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let (_, n) = Self::dfs(root);
        n
    }
    
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        let Some(node) = root else { return (0, None) };

        let (left_d, left) = Self::dfs(node.borrow().left.clone());
        let (right_d, right) = Self::dfs(node.borrow().right.clone());

        if left_d == right_d {
            (left_d + 1, Some(node))
        } else if left_d > right_d {
            (left_d + 1, left)
        } else {
            (right_d + 1, right)
        }
    }
}
