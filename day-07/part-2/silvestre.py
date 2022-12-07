from tool.runners.python import SubmissionPy
from typing import Dict, List

TOTAL_SPACE = 70_000_000
REQUIRED_SPACE = 30_000_000


def _list_dir_size(file_system: Dict, curr: List[str], dir_sizes: Dict):
    total_size = 0
    for key, value in file_system.items():
        if isinstance(value, int):
            total_size += value
        else:
            if not (key in dir_sizes):
                _list_dir_size(file_system[key], [*curr, key], dir_sizes)
            total_size += dir_sizes[(*curr, key)]
    dir_sizes[tuple(curr)] = total_size

class Terminal():

    def __init__(self):
        self.curr = []
        self.file_system = {}

    @staticmethod
    def _get(path: List[str], tree: Dict):        
        if len(path) == 0:
            return tree
        
        for elem in path:
            tree = tree[elem]
        return tree

    def cd(self, path):
        if path == "/":
            self.curr = []
        elif path == "..":
            self.curr.pop()
        elif path in self._get(self.curr, self.file_system).keys():
            self.curr.append(path)

    def add(self, arg1, arg2):
        if arg1 == "dir":
            self._get(self.curr, self.file_system)[arg2] = {}
        else:
            self._get(self.curr, self.file_system)[arg2] = int(arg1)

    def list_dir_size(self):
        dir_sizes = {}
        _list_dir_size(self.file_system, [], dir_sizes)
        return dir_sizes


class SilvestreSubmission(SubmissionPy):

    def run(self, s):
        term = Terminal()
        lines = s.split("\n")
        idx = 0
        while idx < len(lines):
            if lines[idx].startswith("$ cd"):
                term.cd(lines[idx][5:])
                idx += 1
            elif lines[idx].startswith("$ ls"):
                idx += 1
                while idx < len(lines) and not (lines[idx].startswith("$")):
                    term.add(*lines[idx].split(" "))
                    idx += 1
        dir_sizes = term.list_dir_size()
        space_to_find = REQUIRED_SPACE - (TOTAL_SPACE - dir_sizes[tuple()])
        return sorted([v for v in dir_sizes.values() if v > space_to_find])[0]

        


def test_silvestre():
    """
    Run `python -m pytest ./day-07/part-1\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """
""".strip()
        )
        == None
    )
