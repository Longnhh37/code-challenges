// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        dummy.next = head;
        let mut cur = &mut dummy;

        // traverse to node before rev
        for _ in 0..left - 1 {
            cur = cur.next.as_mut().unwrap();
        }

        // reverse
        let mut rev_head = cur.next.take();
        let mut rev_tail = None;
        for _ in 0..right - left + 1 {
            let next = rev_head.as_mut().unwrap().next.take();
            rev_head.as_mut().unwrap().next = rev_tail;
            (rev_tail, rev_head) = (rev_head, next);
        }

        // move cur to end of reversed_list
        cur.next = rev_tail;
        for _ in 0..right - left + 1 {
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = rev_head;
        dummy.next
    }
}
