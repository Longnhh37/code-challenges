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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let v1 = if let Some(n) = &l1 { n.val } else { 0 };
            let v2 = if let Some(n) = &l2 { n.val } else { 0 };

            let total = v1 + v2 + carry;
            carry = total / 10;
            tail.next = Some(Box::new(ListNode::new(total % 10)));

            tail = tail.next.as_mut().unwrap();
            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);
        }

        dummy.next
    }
}
