class Solution:
    def lowestCommonAncestor(
        self, root: "TreeNode", p: "TreeNode", q: "TreeNode"
    ) -> "TreeNode":
        l, r = min(p.val, q.val), max(p.val, q.val)
        node = root

        while node is not None:
            if l > node.val:
                node = node.right
            elif r < node.val:
                node = node.left
            else:
                return node
