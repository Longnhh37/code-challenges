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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::len(&head);
        if len == 0 || len == 1  {
            return None;
        }
        let half = len / 2 - 1;

        let mut cur = &mut head;
        for _ in 0..half {
            cur = &mut cur.as_deref_mut().unwrap().next;
        }

        let next = cur.as_mut().unwrap().next.take();
        cur.as_mut().unwrap().next = next.unwrap().next.take();

        head
    }

    fn len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut res = 0;
        while let Some(node) = head {
            res += 1;
            head = &node.next;
        }
        res
    }
}
