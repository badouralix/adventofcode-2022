from tool.runners.python import SubmissionPy


class DidipSubmission(SubmissionPy):
    def intersects(self, left, right):
        if int(left[0]) > int(right[0]):
            left, right = right, left
        return int(left[1]) >= int(right[0])

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        score = 0
        for entry in s.splitlines():
            left, right = entry.split(',')
            left = left.split('-')
            right = right.split('-')

            score += self.intersects(left, right)
        return score



def test_didip():
    """
    Run `python -m pytest ./day-04/part-2/didip.py` to test the submission.
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
        == 4
    )
