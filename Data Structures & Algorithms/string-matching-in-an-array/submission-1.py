class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        n = len(words)
        res = []

        for i in range(n):
            word = words[i]
            for k in range(n):
                if i == k:
                    continue
                if word in words[k]:
                    res.append(word)
        
        return list(set(res))
        