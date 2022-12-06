from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        marker_length = 14
        for i in range(marker_length, len(s)):
            chars = set(s[i - marker_length : i])
            if len(chars) >= marker_length:
                return i


def test_th_ch():
    """
    Run `python -m pytest ./day-06/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
mjqjpqmgbljsphdztnvjfqwrcgsmlb
""".strip()
        )
        == 19
    )
