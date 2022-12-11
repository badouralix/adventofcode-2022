from tool.runners.python import SubmissionPy
from typing import List
from collections import deque

class Tree:
    def __init__(self):
        self.children: List[Tree] = []
        self.size = 0
        self.parent = None
        self.total_size = 0

    def mkdir(self):
        child = Tree()
        self.children.append(child)
        child.parent = self
        return child

    def compute_total_size(self):
        self.total_size = sum(child.compute_total_size() for child in self.children) + self.size
        return self.total_size
        
class DavidSubmission(SubmissionPy):
    DISK_UNUSED_TARGET = 30000000
    TOTAL_DISK_SPACE = 70000000

    def run(self, s:str):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        root = Tree()
        tree = root
        idx = 0
        while idx < len(lines):
            line = lines[idx]
            if line == "$ cd /":
                idx += 1
                continue
            elif line == "$ ls":
                total_size = 0
                idx += 1
                while idx < len(lines) and not lines[idx].startswith("$"):
                    if not lines[idx].startswith("dir"):
                        size, _ = lines[idx].split(" ")
                        total_size += int(size)
                    idx += 1
                tree.size = total_size
            elif line == "$ cd ..":
                tree = tree.parent
                assert tree is not None
                idx += 1
            else:
                assert line.startswith("$ cd ")
                tree = tree.mkdir()
                idx += 1

        root.compute_total_size()

        result = 70000000
        to_free = root.total_size - (self.TOTAL_DISK_SPACE - self.DISK_UNUSED_TARGET)
        to_visit = [root]
        while to_visit:
            tree = to_visit.pop()
            for child in tree.children:
                to_visit.append(child)
    
            if tree.total_size >= to_free:
                result = min(result, tree.total_size)

        return result
