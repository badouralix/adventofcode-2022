from tool.runners.python import SubmissionPy


class JulesdSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        for i in range(14, len(s)):
            if len(set(s[i-14:i])) == 14:
                return i


def test_julesd():
    """
    Run `python -m pytest ./day-06/part-1/julesd.py` to test the submission.
    """
    assert (
        JulesdSubmission().run(
            """
""".strip()
        )
        == None
    )
