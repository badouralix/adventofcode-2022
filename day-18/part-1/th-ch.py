from tool.runners.python import SubmissionPy


def get_neighbors(x, y, z):
    return [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        cubes = set()
        for cube in s.splitlines():
            cubes.add(tuple(int(x) for x in cube.split(",")))

        nb_intersecting_cubes = 0
        for x, y, z in cubes:
            for neighbor in get_neighbors(x, y, z):
                if neighbor in cubes:
                    nb_intersecting_cubes += 1

        return len(cubes) * 6 - nb_intersecting_cubes


def test_th_ch():
    """
    Run `python -m pytest ./day-18/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
2,2,2
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
