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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut counter = vec![0i32; 201];
        let mut cur = &head;

        while let Some(node) = cur {
            let idx = node.val + 100;
            counter[idx as usize] += 1;
            cur = &node.next;
        }

        let mut dummy = Box::new(ListNode::new(-1));
        let mut cur = &mut dummy;
        for (i, &cnt) in counter.iter().enumerate() {
            if cnt == 1 {
                cur.next = Some(Box::new(ListNode::new(i as i32 - 100)));
                cur = cur.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
