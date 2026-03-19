from collections import Counter

class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        s_cnt = Counter(s)

        for ch in t:
            if ch not in s_cnt or s_cnt[ch] == 0:
                return False
            
            s_cnt[ch] -= 1
        
        for k in s_cnt.values():
            if k != 0:
                return False
                     
        return True
        
