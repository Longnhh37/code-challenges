class Solution:
    def heightChecker(self, heights: List[int]) -> int:
        cnt = 0
        h2 = sorted(heights)
        for i in range(len(h2)):
            if heights[i] != h2[i]:
                cnt += 1
        
        return cnt