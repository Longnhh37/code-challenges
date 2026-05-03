impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut cur = head;
        let mut prev_val: Option<i32> = None;

        while let Some(mut node) = cur {
            cur = node.next.take();

            if prev_val != Some(node.val) {
                prev_val = Some(node.val);
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
