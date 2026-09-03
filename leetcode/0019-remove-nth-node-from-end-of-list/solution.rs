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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let len = Self::length(&head);
        if len < n {
            return head;
        }

        let mut dummy = Box::new(ListNode::new(-1));
        dummy.next = head;
        let mut cur = &mut dummy;

        for _ in 0..len - n {
            cur = cur.next.as_mut().unwrap();
        }
        let mut next = cur.next.take();
        let next2 = next.as_mut().unwrap().next.take();
        cur.next = next2;
        dummy.next
    }

    fn length(mut cur: &Option<Box<ListNode>>) -> i32 {
        let mut cnt = 0;
        while let Some(node) = cur {
            cnt += 1;
            cur = &node.next;
        }
        cnt
    }
}
