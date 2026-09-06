use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        for (i, head) in lists.iter().enumerate() {
            if let Some(node) = head {
                heap.push(Reverse((node.val, i)));
            }
        }

        let mut dummy = Box::new(ListNode::new(-1));
        let mut tail = &mut dummy;

        while let Some(Reverse((_, idx))) = heap.pop() {
            let mut node = lists[idx].take().unwrap();
            if let Some(next) = node.next.take() {
                heap.push(Reverse((next.val, idx)));
                lists[idx] = Some(next);
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}
