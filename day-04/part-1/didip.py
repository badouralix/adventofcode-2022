from tool.runners.python import SubmissionPy


class DidipSubmission(SubmissionPy):
    def overlaps(self, left, right):
        if left[0] > right[0]:
            left, right = right, left
        return left[1] >= right[1] or left[0] == right[0]

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        score = 0
        for entry in s.splitlines():
            left, right = entry.split(',')
            left = list(map(int, left.split('-')))
            right = list(map(int, right.split('-')))

            score += self.overlaps(left, right)
        return score


def test_didip():
    """
    Run `python -m pytest ./day-04/part-1/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
""".strip()
        )
        == 2
    )
