class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        counter = [0] * 26

        if len(ransomNote) > len(magazine):
            return False

        for ch in magazine:
            counter[ord(ch) - ord('a')] += 1

        for ch in ransomNote:
            idx = ord(ch) - ord('a') 
            if counter[idx] == 0:
                return False
            counter[idx] -= 1
        
        return True
