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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {

        let mut dummy = Box::new(ListNode::new(-1));
        dummy.next = head;

        let mut cur = &mut dummy;
        while let Some(mut next_node) = cur.next.take() {
            if next_node.val == val {
                cur.next = next_node.next.take();
            } else {
                cur.next = Some(next_node);
                cur = cur.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
