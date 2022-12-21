from tool.runners.python import SubmissionPy
import numpy as np

SHIFT_BY = 458
SAND_START = 500

def parse_coords(str_):
    x, y = str_.split(",")
    x = int(x) - SHIFT_BY
    y = int(y)
    return x, y


def next_grain_coord(grid, coord):
    x, y = coord

    if x + 1 > grid.shape[0] - 1 or y + 1 > grid.shape[1] - 1:
        return x + 1, y + 1

    if grid[x, y + 1] == 0:
        drop_to = x, y + 1
    else:
        if grid[x - 1, y + 1] == 0:
            drop_to = x - 1, y + 1
        elif grid[x + 1, y + 1] == 0:
            drop_to = x + 1, y + 1
        else:
            return None
    return drop_to


def get_grid_size(s):
    import re
    matchs = re.findall(r"(\w+),(\w+)", s)
    min_x = min(map(int, [x[0] for x in matchs]))
    max_x = max(map(int, [x[0] for x in matchs]))
    max_y = max(map(int, [x[1] for x in matchs]))
    return min_x, max_x, 0, max_y


class YouyounSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        global SHIFT_BY
        # Setup the grid
        np.set_printoptions(threshold=150 * 150, linewidth=200)
        min_x, max_x, min_y, max_y = get_grid_size(s)
        SHIFT_BY = min_x
        grid = np.zeros((max_x - SHIFT_BY + 1, max_y + 1))
        for line in s.split("\n"):
            start = None
            for coords in line.split('->'):
                if start is None:
                    start = parse_coords(coords)
                    continue
                to = parse_coords(coords)
                grid[min(start[0], to[0]):max(start[0], to[0]) + 1, min(start[1], to[1]):max(start[1], to[1]) + 1] = 1.0
                start = to
        # print(str(grid.T).replace("1.", "#").replace("2.", "o").replace("0.", "."))

        grid_old = grid.copy()
        while True:
            start = (SAND_START - SHIFT_BY, 0)
            while True:
                drop_to = next_grain_coord(grid, start)
                if drop_to is None:
                    break
                if drop_to[0] < 0 or drop_to[1] < 0 or drop_to[0] > grid.shape[0] - 1 or drop_to[1] > grid.shape[1] - 1:
                    grid[start[0], start[1]] = 0
                    break
                else:
                    grid[start[0], start[1]] = 0
                    grid[drop_to[0], drop_to[1]] = 2
                start = drop_to
            # print(str(grid.T).replace("1.", "#").replace("2.", "o").replace("0.", "."))
            # print()
            if (grid == grid_old).all():
                break
            else:
                grid_old = grid.copy()
        # print(str(grid.T).replace("1.", "#").replace("2.", "o").replace("0.", "."))
        return (grid == 2).sum()


def test_youyoun():
    """
    Run `python -m pytest ./day-14/part-1/youyoun.py` to test the submission.
    """
    assert (
            YouyounSubmission().run(
                """498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
""".strip()
            )
            == 24
    )


if __name__ == "__main__":
    test_youyoun()
