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
        let mut cur = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let v1 = l1.as_ref().map_or(0, |n| n.val);
            let v2 = l2.as_ref().map_or(0, |n| n.val);

            let sum = v1 + v2 + carry;
            carry = sum / 10;
            cur.next = Some(Box::new(ListNode::new(sum % 10)));

            cur = cur.next.as_mut().unwrap();
            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);
        }

        dummy.next
    }
}
