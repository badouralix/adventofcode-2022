from tool.runners.python import SubmissionPy
import string
from functools import reduce

type_to_priority_map = {
    **{c: i+1 for i, c in enumerate(string.ascii_lowercase)},
    **{c: i+27 for i, c in enumerate(string.ascii_uppercase)}
}

def priority(group: str) -> int:
    badge_type = (set(group[0]) & set(group[1]) & set(group[2])).pop()
    return type_to_priority_map[badge_type]

class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        return sum(priority(lines[n:n+3]) for n in range(0, len(lines), 3))


def test_silvestre():
    """
    Run `python -m pytest ./day-03/part-2\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """
""".strip()
        )
        == None
    )
