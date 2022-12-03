from tool.runners.python import SubmissionPy


class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        score = 0
        lines = s.splitlines()
        for i in range(len(lines) // 3):
            (label, ) = set(lines[3 * i]) & set(lines[3 * i + 1]) & set(lines[3 * i + 2])
            if ord(label) >= 97:
                score += ord(label) - 96
            else:
                score += ord(label) - 64 + 26

        return score



def test_didip():
    """
    Run `python -m pytest ./day-03/part-2/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
""".strip()
        )
        == 70
    )
