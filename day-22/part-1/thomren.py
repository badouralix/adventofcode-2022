import re
from tool.runners.python import SubmissionPy

DIRECTIONS = [(0, 1), (1, 0), (0, -1), (-1, 0)]


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid, instructions = s.split("\n\n")
        grid = grid.splitlines()
        x = 0
        y = next(y for y in range(len(grid[0])) if grid[0][y] == ".")
        direction = 0
        for ins in re.findall(r"(\d+|\D+)", instructions.strip()):
            if ins.isdigit():
                for _ in range(int(ins)):
                    next_x = x + DIRECTIONS[direction][0]
                    next_y = y + DIRECTIONS[direction][1]
                    if direction == 0 and (
                        next_y >= len(grid[next_x]) or grid[next_x][next_y] == " "
                    ):
                        next_y = next(
                            y for (y, c) in enumerate(grid[next_x]) if c != " "
                        )
                    elif direction == 1 and (
                        next_x >= len(grid)
                        or next_y >= len(grid[next_x])
                        or grid[next_x][next_y] == " "
                    ):
                        next_x = next(
                            x for (x, r) in enumerate(grid) if r[next_y] != " "
                        )
                    elif direction == 2 and (next_y < 0 or grid[next_x][next_y] == " "):
                        next_y = next(
                            y
                            for (y, c) in reversed(list(enumerate(grid[next_x])))
                            if c != " "
                        )
                    elif direction == 3 and (
                        next_x < 0
                        or next_y >= len(grid[next_x])
                        or grid[next_x][next_y] == " "
                    ):
                        next_x = next(
                            x
                            for (x, r) in reversed(list(enumerate(grid)))
                            if next_y < len(r) and r[next_y] != " "
                        )

                    if grid[next_x][next_y] == "#":
                        break

                    x = next_x
                    y = next_y
            elif ins == "L":
                direction -= 1
                direction %= len(DIRECTIONS)
            elif ins == "R":
                direction += 1
                direction %= len(DIRECTIONS)
            else:
                raise ValueError(f"unknown instruction: {ins}")

        return 1000 * (x + 1) + 4 * (y + 1) + direction


def test_thomren():
    """
    Run `python -m pytest ./day-22/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"""
        )
        == 6032
    )
