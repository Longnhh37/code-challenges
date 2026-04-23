class Solution:
    def countBits(self, n: int) -> List[int]:
        ans = [0] * (n + 1)
        for i in range(1, n + 1):
            cnt = 0
            j = i

            while j > 0:
                if j % 2 == 1:
                    cnt += 1
                    j -= 1
                else:
                    j /= 2
            ans[i] = cnt

        return ans
