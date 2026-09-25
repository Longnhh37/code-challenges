// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head.as_mut();
        while let Some(node) = cur {
            if let Some(next) = node.next.take() {
                let g = Self::gcd(node.val, next.val);
                let mut gcd_node = Box::new(ListNode::new(g));
                gcd_node.next = Some(next);
                node.next = Some(gcd_node);
                cur = node.next.as_mut().unwrap().next.as_mut();
            } else {
                cur = None;
            }
        }

        head
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}
