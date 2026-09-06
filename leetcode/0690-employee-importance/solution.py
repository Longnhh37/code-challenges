"""
# Definition for Employee.
class Employee:
    def __init__(self, id: int, importance: int, subordinates: List[int]):
        self.id = id
        self.importance = importance
        self.subordinates = subordinates
"""

from collections import deque

class Solution:
    def getImportance(self, employees: List['Employee'], id: int) -> int:
        id_to_index = {}
        for idx, e in enumerate(employees):
            id_to_index[e.id] = idx
        
        res = 0
        q = deque([id])
        while q:
            id = q.popleft()
            idx = id_to_index[id]
            emp = employees[idx]
            res += emp.importance
            for sub in emp.subordinates:
                q.append(sub)

        return res


        
