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
use std::collections::VecDeque;

type TreeLink = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn right_side_view(root: Option<TreeLink>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let Some(root) = root else { return res };

        let mut queue: VecDeque<TreeLink> = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let level_size = queue.len();
            for i in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();

                if i == level_size - 1 {
                    res.push(node_ref.val);
                }

                if let Some(left) = &node_ref.left {
                    queue.push_back(Rc::clone(left));
                }
                if let Some(right) = &node_ref.right {
                    queue.push_back(Rc::clone(right));
                }
            }
        }

        res
    }
}
