# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        cur = head
        seen = set()
        
        while cur is not None:
            if cur in seen:
                return True
            else:
                seen.add(cur)
            cur = cur.next

        return False
