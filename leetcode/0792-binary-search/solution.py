from bisect import bisect_left

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        i = bisect_left(nums, target)

        if i < len(nums) and target == nums[i]:
            return i
        else:
            return -1
        
