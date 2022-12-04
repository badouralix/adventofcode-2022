from tool.runners.python import SubmissionPy


class BebertSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        count = 0
        for line in s.splitlines():
            [p1, p2] = line.split(",")
            [a01, a02] = p1.split("-")
            [b01, b02] = p2.split("-")
            if int(a01) <= int(b02) and int(b01) <= int(a02):
                count += 1
        return count


def test_bebert():
    """
    Run `python -m pytest ./day-04/part-2/bebert.py` to test the submission.
    """
    assert (
            BebertSubmission().run(
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
