from tool.runners.python import SubmissionPy


class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        return sum(
            map(
                lambda x: (x[0] + x[1] - 1) % 3 + 1 + 3 * x[1]
                ,
                map(lambda x: [ord(x.split()[0]) - ord('A'), ord(x.split()[1]) - ord('X')], s.splitlines())  # split the entry and map to 012
            )
        )


def test_didip():
    """
    Run `python -m pytest ./day-02/part-2/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """A Y
B X
C Z
""".strip()
        )
        == 12
    )
