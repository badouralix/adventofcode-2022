from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.split("\n")
        return sum(1 for line in lines if self.overlaps(*self.parse_line(line)))

    @staticmethod
    def overlaps(i1: int, i2: int, j1: int, j2: int) -> bool:
        if j2 >= i1 and j1 <= i2:
            return True
        return False

    @staticmethod
    def parse_line(line):
        pair = line.split(",")
        i1, i2 = [int(x) for x in pair[0].split("-")]
        j1, j2 = [int(x) for x in pair[1].split("-")]
        assert i1 <= i2 and j1 <= j2

        return (i1, i2, j1, j2)


def test_david():
    """
    Run `python -m pytest ./day-04/part-1/david.py` to test the submission.
    """
    assert (
        DavidSubmission().run(
            """
""".strip()
        )
        == None
    )
