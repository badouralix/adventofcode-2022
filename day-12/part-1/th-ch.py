from tool.runners.python import SubmissionPy

from collections import defaultdict
from queue import PriorityQueue


def elevation(value):
    if value == "S":
        value = "a"
    elif value == "E":
        value = "z"
    return ord(value) - ord("a")


def dijkstra(grid, x_s, y_s, progress, end=None):
    D = defaultdict(lambda: float("inf"))
    D[(x_s, y_s)] = 0

    pq = PriorityQueue()
    pq.put((0, (x_s, y_s)))

    visited = set()

    while not pq.empty():
        _, (x, y) = pq.get()
        visited.add((x, y))
        if grid[y][x] == end:
            return D[(x, y)]

        for dx, dy in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
            xx, yy = x + dx, y + dy
            if (
                0 <= yy < len(grid)
                and 0 <= xx < len(grid[yy])
                and progress(elevation(grid[y][x]), elevation(grid[yy][xx]))
            ):
                if (xx, yy) not in visited:
                    old_cost = D[(xx, yy)]
                    new_cost = D[(x, y)] + 1
                    if new_cost < old_cost:
                        pq.put((new_cost, (xx, yy)))
                        D[(xx, yy)] = new_cost

    return D


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[c for c in line] for line in s.splitlines()]

        for y_s in range(len(grid)):
            try:
                x_s = grid[y_s].index("S")
                break
            except ValueError:
                pass

        return dijkstra(
            grid,
            x_s,
            y_s,
            progress=lambda current, next_val: next_val <= current + 1,
            end="E",
        )


def test_th_ch():
    """
    Run `python -m pytest ./day-12/part-1/th-ch.py` to test the submission.
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
        == 31
    )
