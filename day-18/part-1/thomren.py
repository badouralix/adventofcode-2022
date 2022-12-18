from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        droplets = {tuple(map(int, line.split(","))) for line in s.splitlines()}
        faces = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        return sum(
            (x + dx, y + dy, z + dz) not in droplets
            for (dx, dy, dz) in faces
            for (x, y, z) in droplets
        )


def test_thomren():
    """
    Run `python -m pytest ./day-18/part-1/thomren.py` to test the submission.
    """
    assert ThomrenSubmission().run("1,1,1\n2,1,1") == 10

    assert (
        ThomrenSubmission().run(
            """2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
""".strip()
        )
        == 64
    )
