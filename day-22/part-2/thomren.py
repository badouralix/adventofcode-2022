import re
from typing import Dict, List, Tuple
from tool.runners.python import SubmissionPy

DIRECTIONS = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # right, down, left, up
CURSORS = [">", "v", "<", "^"]

# TODO: compute FACE_SIZE and FACE_DIR_MAP from the input instead of hardcoding
FACE_SIZE = 50
# FACE_DIR_MAP[face][direction] = next_face, next_direction, inverse_edge_pos
FACE_DIR_MAP: Dict[Tuple[int, int], List[Tuple[Tuple[int, int], int, bool]]] = {
    (0, 1): [
        ((0, 2), 0, False),
        ((1, 1), 1, False),
        ((2, 0), 0, True),
        ((3, 0), 0, False),
    ],
    (0, 2): [
        ((2, 1), 2, True),
        ((1, 1), 2, False),
        ((0, 1), 2, False),
        ((3, 0), 3, False),
    ],
    (1, 1): [
        ((0, 2), 3, False),
        ((2, 1), 1, False),
        ((2, 0), 1, False),
        ((0, 1), 3, False),
    ],
    (2, 0): [
        ((2, 1), 0, False),
        ((3, 0), 1, False),
        ((0, 1), 0, True),
        ((1, 1), 0, False),
    ],
    (2, 1): [
        ((0, 2), 2, True),
        ((3, 0), 2, False),
        ((2, 0), 2, False),
        ((1, 1), 3, False),
    ],
    (3, 0): [
        ((2, 1), 3, False),
        ((0, 2), 1, False),
        ((0, 1), 1, False),
        ((2, 0), 3, False),
    ],
}


class ThomrenSubmission(SubmissionPy):
    def run(self, s, debug=False):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid, instructions = s.split("\n\n")
        grid = {
            (x, y): c
            for (x, line) in enumerate(grid.splitlines())
            for (y, c) in enumerate(line)
            if c in ".#"
        }
        x = 0
        y = next(y for y in range(1000) if grid.get((x, y), " ") in ".#")
        direction = 0
        grid[(x, y)] = CURSORS[direction]
        for ins in re.findall(r"(\d+|\D+)", instructions.strip()):
            if ins.isdigit():
                for _ in range(int(ins)):
                    next_x = x + DIRECTIONS[direction][0]
                    next_y = y + DIRECTIONS[direction][1]
                    next_direction = direction
                    if (next_x, next_y) not in grid:
                        next_x, next_y, next_direction = wrap(grid, x, y, direction)

                    if grid[(next_x, next_y)] == "#":
                        break

                    x = next_x
                    y = next_y
                    direction = next_direction

                    if debug:
                        grid[(x, y)] = CURSORS[direction]
            elif ins == "L":
                direction -= 1
                direction %= len(DIRECTIONS)
            elif ins == "R":
                direction += 1
                direction %= len(DIRECTIONS)
            else:
                raise ValueError(f"unknown instruction: {ins}")

            if debug:
                grid[(x, y)] = CURSORS[direction]

        if debug:
            print(
                "\n".join(
                    "".join(grid.get((x, y), " ") for y in range(FACE_SIZE * 4))
                    for x in range(FACE_SIZE * 4)
                )
            )
            print(x, y, direction)
        return 1000 * (x + 1) + 4 * (y + 1) + direction


def wrap(grid, x, y, direction):
    fx = x // FACE_SIZE
    fy = y // FACE_SIZE
    edge_pos = (x if direction % 2 == 0 else y) % FACE_SIZE
    next_face, direction, inverse_edge_pos = FACE_DIR_MAP[(fx, fy)][direction]
    if inverse_edge_pos:
        edge_pos = FACE_SIZE - edge_pos - 1
    next_x, next_y = get_coords(grid, edge_pos, next_face, direction)
    return next_x, next_y, direction


def get_coords(grid, edge_pos, next_face, direction):
    if direction == 0:
        next_x = edge_pos + next_face[0] * FACE_SIZE
        next_y = next(y for y in range(1000) if (next_x, y) in grid)
    elif direction == 1:
        next_y = edge_pos + next_face[1] * FACE_SIZE
        next_x = next(x for x in range(1000) if (x, next_y) in grid)
    elif direction == 2:
        next_x = edge_pos + next_face[0] * FACE_SIZE
        next_y = next(y for y in range(150, -1, -1) if (next_x, y) in grid)
    elif direction == 3:
        next_y = edge_pos + next_face[1] * FACE_SIZE
        next_x = next(x for x in range(200, -1, -1) if (x, next_y) in grid)
    else:
        raise ValueError(f"invalid direction: {direction}")

    return next_x, next_y


def test_wrap():
    s = open(__file__.replace("part-2", "input").replace("py", "txt")).read()
    grid, _ = s.split("\n\n")
    grid = {
        (x, y): c
        for (x, line) in enumerate(grid.splitlines())
        for (y, c) in enumerate(line)
        if c in ".#"
    }
    assert wrap(grid, 0, 50, 2) == (149, 0, 0)
    assert wrap(grid, 0, 84, 3) == (184, 0, 0)
    assert wrap(grid, 6, 149, 0) == (143, 99, 2)
    assert wrap(grid, 49, 103, 1) == (53, 99, 2)
    assert wrap(grid, 0, 147, 3) == (199, 47, 3)
    assert wrap(grid, 64, 99, 0) == (49, 114, 3)
    assert wrap(grid, 64, 50, 2) == (100, 14, 1)
    assert wrap(grid, 100, 0, 2) == (49, 50, 0)
    assert wrap(grid, 100, 0, 3) == (50, 50, 0)
    assert wrap(grid, 132, 99, 0) == (17, 149, 2)
    assert wrap(grid, 149, 99, 1) == (199, 49, 2)
    assert wrap(grid, 156, 49, 0) == (149, 56, 3)
    assert wrap(grid, 199, 23, 1) == (0, 123, 1)
    assert wrap(grid, 187, 0, 2) == (0, 87, 1)


def test_thomren():
    """
    Run `python -m pytest ./day-22/part-1/thomren.py` to test the submission.
    """
    global FACE_SIZE, FACE_DIR_MAP
    fs = FACE_SIZE
    fdm = FACE_DIR_MAP

    FACE_SIZE = 4
    FACE_DIR_MAP = {
        (0, 2): [
            ((2, 3), 2, True),
            ((1, 3), 1, False),
            ((1, 1), 1, False),
            ((1, 0), 1, True),
        ],
        (1, 0): [
            ((1, 1), 0, False),
            ((2, 2), 3, True),
            ((2, 3), 3, True),
            ((0, 2), 1, True),
        ],
        (1, 1): [
            ((1, 2), 0, False),
            ((2, 2), 0, True),
            ((1, 0), 2, False),
            ((0, 2), 0, False),
        ],
        (1, 2): [
            ((2, 3), 1, True),
            ((2, 2), 1, False),
            ((1, 1), 2, False),
            ((0, 2), 3, False),
        ],
        (2, 2): [
            ((2, 3), 0, False),
            ((1, 0), 3, True),
            ((1, 1), 3, True),
            ((1, 2), 3, False),
        ],
        (2, 3): [
            ((0, 2), 2, True),
            ((1, 0), 0, False),
            ((2, 2), 2, True),
            ((0, 2), 2, True),
        ],
    }

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
        == 5031
    )

    FACE_SIZE = fs
    FACE_DIR_MAP = fdm
