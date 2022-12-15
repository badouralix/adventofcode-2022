from tool.runners.python import SubmissionPy

from collections import defaultdict


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        blocked = defaultdict(set)
        nb_sand_units = 0
        y_max = 0

        for line in s.splitlines():
            rock_limits = line.split(" -> ")
            for i in range(len(rock_limits) - 1):
                x1, y1 = map(int, rock_limits[i].split(","))
                x2, y2 = map(int, rock_limits[i + 1].split(","))
                if x1 == x2:
                    for y in range(min(y1, y2), max(y1, y2) + 1):
                        blocked[x1].add(y)
                else:
                    for x in range(min(x1, x2), max(x1, x2) + 1):
                        blocked[x].add(y1)
                y_max = max(y_max, y1, y2)

        y_max += 2

        while True:
            x = 500
            y = min(blocked[x]) - 1
            while y < y_max - 1:
                if y + 1 not in blocked[x]:
                    y += 1
                elif y + 1 not in blocked[x - 1]:
                    x -= 1
                    y += 1
                elif y + 1 not in blocked[x + 1]:
                    x += 1
                    y += 1
                else:
                    break

            if 0 in blocked[500]:
                return nb_sand_units

            blocked[x].add(y)
            nb_sand_units += 1


def test_th_ch():
    """
    Run `python -m pytest ./day-14/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
""".strip()
        )
        == 93
    )
