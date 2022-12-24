from tool.runners.python import SubmissionPy

import re


def next_pos(x, y, direction):
    if direction == "right":
        return (x + 1, y)
    elif direction == "bottom":
        return (x, y + 1)
    elif direction == "left":
        return (x - 1, y)
    else:
        return (x, y - 1)


def wrap_around(x, y, board, direction):
    if direction == "right":
        return (next(i for i in range(len(board[y])) if board[y][i] != " "), y)
    elif direction == "bottom":
        return (x, next(i for i in range(len(board)) if board[i][x] != " "))
    elif direction == "left":
        return (
            next(i for i in range(len(board[y]) - 1, -1, -1) if board[y][i] != " "),
            y,
        )
    else:
        return (x, next(i for i in range(len(board) - 1, -1, -1) if board[i][x] != " "))


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        board_str, instructions = s.split("\n\n")
        board = [[char for char in line] for line in board_str.splitlines()]
        width = max(len(line) for line in board)
        for y in range(len(board)):
            board[y].extend([" "] * (width - len(board[y])))
        x, y = next(i for i, tile in enumerate(board[0]) if tile == "."), 0
        direction = "right"
        for instruction in re.split(r"(\d+)", instructions)[1:-1]:
            if instruction.isnumeric():
                for _ in range(int(instruction)):
                    xx, yy = next_pos(x, y, direction)
                    try:
                        should_wrap_around = board[yy][xx] == " "
                    except IndexError:
                        should_wrap_around = True
                    if should_wrap_around:
                        xx, yy = wrap_around(xx, yy, board, direction)
                    if board[yy][xx] == "#":
                        break
                    else:
                        x, y = xx, yy
            else:
                if instruction == "R":
                    if direction == "right":
                        direction = "bottom"
                    elif direction == "bottom":
                        direction = "left"
                    elif direction == "left":
                        direction = "top"
                    else:
                        direction = "right"
                else:
                    if direction == "right":
                        direction = "top"
                    elif direction == "bottom":
                        direction = "right"
                    elif direction == "left":
                        direction = "bottom"
                    else:
                        direction = "left"

        if direction == "right":
            facing = 0
        elif direction == "bottom":
            facing = 1
        elif direction == "left":
            facing = 2
        else:
            facing = 3

        return 1000 * (y + 1) + 4 * (x + 1) + facing


def test_th_ch():
    """
    Run `python -m pytest ./day-22/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
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

10R5L5R10L4R5L5"""
        )
        == 6032
    )
