class Solution:
    def floodFill(
        self, image: list[list[int]], sr: int, sc: int, color: int
    ) -> list[list[int]]:
        m = len(image)
        n = len(image[0])
        start_value = image[sr][sc]

        if start_value == color:
            return image

        def dfs(r, c):
            if image[r][c] != start_value:
                return
            
            image[r][c] = color

            for dr, dc in [(1,0),(-1,0),(0,1),(0,-1)]:
                nr, nc = r + dr, c + dc
                if 0 <= nr < m and 0 <= nc < n:
                    dfs(nr, nc)
        
        dfs(sr, sc)
        return image        

