import random
import string

class Robot:
    __names = [
        a + b + f"{i:03}"
        for a in string.ascii_uppercase
        for b in string.ascii_uppercase
        for i in range(1000)
    ]
    random.shuffle(__names)

    def __init__(self):
        if not self.__class__.__names:
            raise ValueError("No more unique names")

        self.name = self.__class__.__names.pop()

    def reset(self):
        self.name = self.__class__.__names.pop()