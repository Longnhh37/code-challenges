class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        min_price = float('inf')
        max_profit = 0

        for price in prices:
            min_price = price if price < min_price else min_price
            max_profit = price - min_price if price - min_price > max_profit else max_profit

        return max_profit
        
