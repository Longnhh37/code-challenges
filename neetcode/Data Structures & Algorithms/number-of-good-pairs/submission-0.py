class Solution:
    def numIdenticalPairs(self, nums: List[int]) -> int:
        res = 0
        counter = [0 for _ in range(101)]

        for num in nums:
            counter[num] += 1
        for cnt in counter:
            res += cnt * (cnt - 1) // 2

        return res


        