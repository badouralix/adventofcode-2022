from tool.runners.python import SubmissionPy

SAND_START = 500


def parse_coords(str_):
    x, y = str_.split(",")
    x = int(x)
    y = int(y)
    return x, y


def next_grain_coord(wall: set, sand: set, coord, max_depth: int):
    x, y = coord
    if y + 1 >= max_depth:
        return None

    if (x, y + 1) not in sand and (x, y + 1) not in wall:
        drop_to = x, y + 1
    else:
        if (x - 1, y + 1) not in sand and (x - 1, y + 1) not in wall:
            drop_to = x - 1, y + 1
        elif (x + 1, y + 1) not in sand and (x + 1, y + 1) not in wall:
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
        sand_points = set()
        wall_points = set()
        min_x, max_x, min_y, max_y = get_grid_size(s)
        for line in s.split("\n"):
            start = None
            for coords in line.split('->'):
                if start is None:
                    start = parse_coords(coords)
                    continue
                to = parse_coords(coords)
                for i in range(min(start[0], to[0]), max(start[0], to[0]) + 1):
                    for j in range(min(start[1], to[1]), max(start[1], to[1]) + 1):
                        wall_points.add((i, j))
                start = to

        while True:
            start = (SAND_START, 0)
            while True:
                drop_to = next_grain_coord(wall_points, sand_points, start, max_y + 2)
                if drop_to is None:
                    break
                else:
                    if start in sand_points:
                        sand_points.remove(start)
                    sand_points.add(drop_to)
                start = drop_to
            if (500, 1) in sand_points and (499, 1) in sand_points and (501, 1) in sand_points:
                break
        return len(sand_points) + 1


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
            == 93
    )


if __name__ == "__main__":
    test_youyoun()
