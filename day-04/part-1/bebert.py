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
            a1 = int(a01)
            a2 = int(a02)
            b1 = int(b01)
            b2 = int(b02)
            if a1 >= b1 and a2 <= b2 or b1 >= a1 and b2 <= a2:
                count += 1
        return count


def test_bebert():
    """
    Run `python -m pytest ./day-04/part-1/bebert.py` to test the submission.
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
            == 2
    )
