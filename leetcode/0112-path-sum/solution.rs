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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::dfs(&root, 0, target_sum)
    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut cur_sum: i32, target_sum: i32) -> bool {
        let Some(node) = root else { return false; };
        let n = node.borrow();
        cur_sum += n.val;

        if n.left.is_none() && n.right.is_none() {
            cur_sum == target_sum
        } else {
            Self::dfs(&n.left, cur_sum, target_sum) || Self::dfs(&n.right, cur_sum, target_sum)
        }
    }
}
