from tool.runners.python import SubmissionPy
import string

type_to_priority_map = {
    **{c: i+1 for i, c in enumerate(string.ascii_lowercase)},
    **{c: i+27 for i, c in enumerate(string.ascii_uppercase)}
}

def priority(rucksack: str) -> int:
    misplaced_type = (set(rucksack[:len(rucksack)//2]) & set(rucksack[len(rucksack)//2:])).pop()
    return type_to_priority_map[misplaced_type]

class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        return sum(priority(rucksack) for rucksack in s.split("\n"))


def test_silvestre():
    """
    Run `python -m pytest ./day-03/part-1\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """""".strip()
        )
        == None
    )
