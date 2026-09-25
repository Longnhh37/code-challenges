// Definition for a QuadTree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: bool,
//     pub is_leaf: bool,
//     pub top_left: Option<Rc<RefCell<Node>>>,
//     pub top_right: Option<Rc<RefCell<Node>>>,
//     pub bottom_left: Option<Rc<RefCell<Node>>>,
//     pub bottom_right: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: bool, is_leaf: bool) -> Self {
//         Node {
//             val,
//             is_leaf,
//             top_left: None,
//             top_right: None,
//             bottom_left: None,
//             bottom_right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

type Link = Rc<RefCell<Node>>;

impl Solution {
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Link> {
        let n = grid.len();
        Some(Self::dfs(&grid, n, 0, 0))
    }
    
    fn dfs(grid: &[Vec<i32>], n: usize, r: usize, c: usize) -> Link {
        let node = Rc::new(RefCell::new(Node::new(grid[r][c] == 1, false)));

        if n == 1 {
            node.borrow_mut().is_leaf = true;
            return node;
        }

        let mid = n / 2;
        let tl = Self::dfs(grid, mid, r, c);
        let tr = Self::dfs(grid, mid, r, c + mid);
        let bl = Self::dfs(grid, mid, r + mid, c);
        let br = Self::dfs(grid, mid, r + mid, c + mid);

        let tlv = tl.borrow().val;
        let trv = tr.borrow().val;
        let blv = bl.borrow().val;
        let brv = br.borrow().val;

        let tl_leaf = tl.borrow().is_leaf;
        let tr_leaf = tr.borrow().is_leaf;
        let bl_leaf = bl.borrow().is_leaf;
        let br_leaf = br.borrow().is_leaf;

        if tl_leaf && tr_leaf && bl_leaf && br_leaf
            && tlv == trv && tlv == blv && tlv == brv {
            node.borrow_mut().is_leaf = true;
            return node;
        } 

        {
        let mut nb = node.borrow_mut();
        nb.top_left = Some(tl);
        nb.top_right = Some(tr);
        nb.bottom_left = Some(bl);
        nb.bottom_right = Some(br);
        }
        node
    }
}