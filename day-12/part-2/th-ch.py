from tool.runners.python import SubmissionPy

from importlib import import_module

part1 = import_module("day-12.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[c for c in line] for line in s.splitlines()]

        starting_points = set(
            (x_s, y_s)
            for y_s in range(len(grid))
            for x_s in range(len(grid[y_s]))
            if part1.elevation(grid[y_s][x_s]) == 0
        )
        for y_end in range(len(grid)):
            try:
                x_end = grid[y_end].index("E")
                break
            except ValueError:
                pass

        return min(
            dist
            for (x, y), dist in part1.dijkstra(
                grid,
                x_end,
                y_end,
                progress=lambda current, next_val: next_val >= current - 1,
            ).items()
            if (x, y) in starting_points
        )


def test_th_ch():
    """
    Run `python -m pytest ./day-12/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
""".strip()
        )
        == 29
    )
