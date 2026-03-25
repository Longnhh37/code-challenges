from collections import Counter

class Solution:
    def longestPalindrome(self, s: str) -> int:
        avail_chars = Counter(s)
        length = sum(v // 2 * 2 for v in avail_chars.values())
        
        return length + (length < len(s))
