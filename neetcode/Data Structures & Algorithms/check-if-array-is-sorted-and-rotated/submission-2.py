class Solution:
    def check(self, nums: List[int]) -> bool:
        n = len(nums)
        found = 0
        i = 0
        for i in range(n):
            if nums[i] > nums[(i + n + 1) % n]:
                found += 1

        return found == 0 or found == 1
        

        