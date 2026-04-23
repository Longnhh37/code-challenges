class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        a = min(strs)
        b = max(strs)

        i = 0
        while i < len(a) and a[i] == b[i]:
            i += 1
        
        return a[:i]
