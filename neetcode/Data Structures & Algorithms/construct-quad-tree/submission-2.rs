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
        let mut node = Node::new(grid[r][c] == 1, false);

        if n == 1 {
            node.is_leaf = true;
            return Rc::new(RefCell::new(node));
        }

        let mid = n / 2;
        let tl = Self::dfs(grid, mid, r, c);
        let tr = Self::dfs(grid, mid, r, c + mid);
        let bl = Self::dfs(grid, mid, r + mid, c);
        let br = Self::dfs(grid, mid, r + mid, c + mid);

        let (tl_leaf, tlv) = { let b = tl.borrow(); (b.is_leaf, b.val) };
        let (tr_leaf, trv) = { let b = tr.borrow(); (b.is_leaf, b.val) };
        let (bl_leaf, blv) = { let b = bl.borrow(); (b.is_leaf, b.val) };
        let (br_leaf, brv) = { let b = br.borrow(); (b.is_leaf, b.val) };

        if tl_leaf && tr_leaf && bl_leaf && br_leaf
            && tlv == trv && tlv == blv && tlv == brv {
            node.is_leaf = true;
        }  else {
            node.top_left = Some(tl);
            node.top_right = Some(tr);
            node.bottom_left = Some(bl);
            node.bottom_right = Some(br);
        }

        Rc::new(RefCell::new(node))
    }
}