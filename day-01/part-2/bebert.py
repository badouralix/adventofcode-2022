from tool.runners.python import SubmissionPy


class BebertSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        return sum(sorted(sum(int(line) for line in group.splitlines()) for group in s.split("\n\n"))[-3:])


def test_bebert():
    """
    Run `python -m pytest ./day-01/part-1/bebert.py` to test the submission.
    """
    assert (
            BebertSubmission().run(
                """1
    2

    2
    1

    5""".strip()
            )
            == 5
    )
