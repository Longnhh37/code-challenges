impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut cur = head;

        while let Some(mut node) = cur {
            let mut next = node.next.take();
            node.next = prev;
            prev = Some(node);
            cur = next;
        }

        prev
    }
}
