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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.as_ref().unwrap();
        let n = node.borrow();
        match n.val {
            0 => false,
            1 => true,
            2 => Self::dfs(&n.left) || Self::dfs(&n.right),
            3 => Self::dfs(&n.left) && Self::dfs(&n.right),
            _ => unreachable!(),
        }
    }
}
