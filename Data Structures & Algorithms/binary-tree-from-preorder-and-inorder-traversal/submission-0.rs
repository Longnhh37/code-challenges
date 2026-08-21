// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

type Link = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Link> {
        let idx_map: HashMap<i32, usize> = inorder.iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();

        let mut pre_idx = 0;
        Self::helper(&preorder, &idx_map, &mut pre_idx, 0, inorder.len() as i32 - 1)
    }

    fn helper(
        preorder: &[i32],
        idx_map: &HashMap<i32, usize>,
        pre_idx: &mut usize,
        in_left: i32,
        in_right: i32,
    ) -> Option<Link> {
        if in_left > in_right {
            return None;
        }

        let root_val = preorder[*pre_idx];
        *pre_idx += 1;

        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let root_idx = idx_map[&root_val] as i32;

        root.borrow_mut().left = Self::helper(preorder, idx_map, pre_idx, in_left, root_idx - 1);
        root.borrow_mut().right = Self::helper(preorder, idx_map, pre_idx, root_idx + 1, in_right);

        Some(root)
    }
}
