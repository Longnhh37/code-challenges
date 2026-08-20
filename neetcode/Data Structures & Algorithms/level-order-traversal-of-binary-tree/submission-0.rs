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
    pub fn level_order(root: Option<TreeLink>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut queue: VecDeque<TreeLink> = VecDeque::from([root.unwrap()]);

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level_vec = Vec::new();
            for i in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();
                level_vec.push(node_ref.val);

                let left = node_ref.left.clone();
                if left.is_some() {
                    queue.push_back(left.unwrap());
                }
                let right = node_ref.right.clone();
                if right.is_some() {
                    queue.push_back(right.unwrap());
                }
            }
            res.push(level_vec);
        }

        res
    }
}
