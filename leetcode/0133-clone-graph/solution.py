"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

from typing import Optional
class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if not node:
            return None
            
        cloned: dict[Node, Node] = {}

        def dfs(u: Node) -> Node:
            if u in cloned:
                return cloned[u]
            
            copy = Node(u.val)
            cloned[u] = copy

            for v in u.neighbors:
                copy.neighbors.append(dfs(v))
            

            return copy

        return dfs(node)
