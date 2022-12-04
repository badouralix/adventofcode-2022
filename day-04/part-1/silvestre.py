from tool.runners.python import SubmissionPy
from typing import Tuple

def is_subset(from1: int, to1: int, from2: int, to2: int) -> bool:
    return (
        ((from1 <= from2) and (to1 >= to2))
        or ((from2 <= from1) and (to2 >= to1))
    )

def parse_line(line: str) -> Tuple[int, int, int, int]:
    return (int(el) for r in line.split(",") for el in r.split("-"))

class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        return sum(is_subset(*parse_line(line)) for line in lines)


def test_silvestre():
    """
    Run `python -m pytest ./day-04/part-1\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """
""".strip()
        )
        == None
    )
