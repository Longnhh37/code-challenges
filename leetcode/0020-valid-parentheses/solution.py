class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        match = {
            ")": "(",
            "]": "[",
            "}": "{",
        }

        for ch in s:
            if ch in match:
                if not stack or match.get(ch) != stack.pop():
                    return False
            else:
                stack.append(ch)

        return not stack

