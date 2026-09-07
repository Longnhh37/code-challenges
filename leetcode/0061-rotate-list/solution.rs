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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut values = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            values.push(node.val);
            cur = &node.next;
        }

        let n = values.len();
        let i = (k as usize % n);
        values.reverse();
        values[..i].reverse();
        values[i..].reverse();

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        for v in values {
            tail.next = Some(Box::new(ListNode::new(v)));
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}
