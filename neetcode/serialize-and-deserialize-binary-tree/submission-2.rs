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

use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<TreeNode>>;

struct Codec;

impl Codec {
    fn new() -> Self {
        Self
    }

    fn serialize(&self, root: Option<Link>) -> String {
        let mut res = String::new();
        Self::pre(&root, &mut res);
        res.pop();
        res
    }

    fn pre(root: &Option<Link>, res: &mut String) {
        match root {
            None => res.push_str("#,"),
            Some(node) => {
                let node = node.borrow();
                res.push_str(&node.val.to_string());
                res.push(',');
                Self::pre(&node.left, res);
                Self::pre(&node.right, res);
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Link> {
        let mut it = data.split(',');
        Self::build(&mut it)
    }

    fn build(it: &mut std::str::Split<'_, char>) -> Option<Link> {
        let tok = it.next()?;
        if tok == "#" {
            return None;
        }
        let val: i32 = tok.parse().unwrap();
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().left = Self::build(it);
        node.borrow_mut().right = Self::build(it);
        Some(node)
    }
}
