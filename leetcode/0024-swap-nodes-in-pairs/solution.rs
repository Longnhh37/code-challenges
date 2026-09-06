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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = &mut dummy;

        loop {
            let check = &cur.next;
            if check.is_some() && check.as_ref().unwrap().next.is_some() {
                let rest = cur.next.as_mut().unwrap().next.as_mut().unwrap().next.take();   
                let pair = cur.next.take();
                cur.next = Self::swap(pair);

                for _ in 0..2 {
                    cur = cur.next.as_mut().unwrap();
                }
                cur.next = rest;
            } else {
                break;
            }
        }

        dummy.next
    }

    fn swap(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut next = head.as_mut().unwrap().next.take(); 
        next.as_mut().unwrap().next = head;
        next
    }
}
