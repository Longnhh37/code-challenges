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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::dfs(root, &mut res);
        res
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        let Some(node) = root else { return; };
        let n = node.borrow();
        Self::dfs(n.left.clone(), res);
        res.push(n.val);
        Self::dfs(n.right.clone(), res);
    }
}
