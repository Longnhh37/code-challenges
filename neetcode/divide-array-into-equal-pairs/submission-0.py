class Solution:
    def divideArray(self, nums: List[int]) -> bool:
        counter = Counter(nums)
        for (_, cnt) in counter.items():
            if cnt % 2 == 1:
                return False
        return True
        