from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        for i in range(4, len(s)):
            chars = set(s[i - 4 : i])
            if len(chars) >= 4:
                return i


def test_th_ch():
    """
    Run `python -m pytest ./day-06/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
mjqjpqmgbljsphdztnvjfqwrcgsmlb
""".strip()
        )
        == 7
    )
