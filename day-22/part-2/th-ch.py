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


def wrap_around(
    x,
    y,
    xx,
    yy,
    board,
    direction,
    get_cube_face,
    next_faces,
    origin_by_face,
    region_size,
):
    current_face = get_cube_face(x, y)
    next_face, next_direction, invert_pos = next_faces[current_face][direction]

    origin_x_current_face, origin_y_current_face = origin_by_face[current_face]
    x_relative_to_current_face = x - origin_x_current_face
    y_relative_to_current_face = y - origin_y_current_face

    origin_x_next_face, origin_y_next_face = origin_by_face[next_face]

    offset = (
        y_relative_to_current_face
        if direction in ["right", "left"]
        else x_relative_to_current_face
    )

    if next_direction == "right":
        yyy = origin_y_next_face + (region_size - 1 - offset if invert_pos else offset)
        xxx = origin_x_next_face
        return (xxx, yyy, next_direction)
    elif next_direction == "bottom":
        xxx = origin_x_next_face + (region_size - 1 - offset if invert_pos else offset)
        yyy = origin_y_next_face
        return (xxx, yyy, next_direction)
    elif next_direction == "left":
        yyy = origin_y_next_face + (region_size - 1 - offset if invert_pos else offset)
        xxx = origin_x_next_face + region_size - 1
        return (xxx, yyy, next_direction)
    else:  # top
        xxx = origin_x_next_face + (region_size - 1 - offset if invert_pos else offset)
        yyy = origin_y_next_face + region_size - 1
        return (xxx, yyy, next_direction)


REGION_SIZE = 50


def get_cube_face_for_input(x, y):
    xx, yy = x // REGION_SIZE, y // REGION_SIZE
    if yy == 0 and xx == 1:
        return 1
    elif yy == 0 and xx == 2:
        return 2
    elif yy == 1 and xx == 1:
        return 3
    elif yy == 2 and xx == 0:
        return 4
    elif yy == 2 and xx == 1:
        return 5
    else:
        return 6


origin_by_face_for_input = {
    1: (1 * REGION_SIZE, 0),
    2: (2 * REGION_SIZE, 0),
    3: (1 * REGION_SIZE, 1 * REGION_SIZE),
    4: (0, 2 * REGION_SIZE),
    5: (1 * REGION_SIZE, 2 * REGION_SIZE),
    6: (0, 3 * REGION_SIZE),
}

# Use: next_faces[face][direction] = next face, next direction, invert_pos
next_faces_for_input = {
    1: {
        "right": (2, "right", False),
        "bottom": (3, "bottom", False),
        "left": (4, "right", True),
        "top": (6, "right", False),
    },
    2: {
        "right": (5, "left", True),
        "bottom": (3, "left", False),
        "left": (1, "left", False),
        "top": (6, "top", False),
    },
    3: {
        "right": (2, "top", False),
        "bottom": (5, "bottom", False),
        "left": (4, "bottom", False),
        "top": (1, "top", False),
    },
    4: {
        "right": (5, "right", False),
        "bottom": (6, "bottom", False),
        "left": (1, "right", True),
        "top": (3, "right", False),
    },
    5: {
        "right": (2, "left", True),
        "bottom": (6, "left", False),
        "left": (4, "left", False),
        "top": (3, "top", False),
    },
    6: {
        "right": (5, "top", False),
        "bottom": (2, "bottom", False),
        "left": (1, "bottom", False),
        "top": (4, "top", False),
    },
}


class ThChSubmission(SubmissionPy):
    def run(
        self,
        s,
        next_faces=next_faces_for_input,
        get_cube_face=get_cube_face_for_input,
        origin_by_face=origin_by_face_for_input,
        region_size=REGION_SIZE,
    ):
        """
        :param s: input in string format
        :return: solution flag

        Input has the form:
            1122
            1122
            33
            33
          4455
          4455
          66
          66

        So the following faces are opposed:
        1 5
        3 6
        2 4
        """
        board = []
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
                    next_direction = direction
                    try:
                        should_wrap_around = board[yy][xx] == " "
                    except IndexError:
                        should_wrap_around = True
                    if should_wrap_around:
                        xx, yy, next_direction = wrap_around(
                            x,
                            y,
                            xx,
                            yy,
                            board,
                            direction,
                            get_cube_face,
                            next_faces,
                            origin_by_face,
                            region_size,
                        )
                    if board[yy][xx] == "#":
                        break
                    else:
                        x, y = xx, yy
                        direction = next_direction
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
    Run `python -m pytest ./day-22/part-2/th-ch.py` to test the submission.

      1
    234
      56
    """
    # Use: next_faces[face][direction] = next face, next direction, invert_pos
    region_size = 4
    next_faces = {
        1: {
            "right": (6, "left", True),
            "bottom": (4, "bottom", False),
            "left": (3, "bottom", False),
            "top": (2, "bottom", True),
        },
        2: {
            "right": (3, "right", False),
            "bottom": (5, "top", True),
            "left": (6, "top", True),
            "top": (1, "bottom", True),
        },
        3: {
            "right": (4, "right", False),
            "bottom": (5, "right", True),
            "left": (2, "left", False),
            "top": (1, "right", False),
        },
        4: {
            "right": (6, "bottom", True),
            "bottom": (5, "bottom", False),
            "left": (3, "left", False),
            "top": (1, "top", False),
        },
        5: {
            "right": (6, "right", False),
            "bottom": (2, "top", True),
            "left": (3, "top", True),
            "top": (4, "top", False),
        },
        6: {
            "right": (1, "left", True),
            "bottom": (2, "right", True),
            "left": (5, "left", False),
            "top": (4, "left", True),
        },
    }
    origin_by_face = {
        1: (2 * region_size, 0),
        2: (0, 1 * region_size),
        3: (1 * region_size, 1 * region_size),
        4: (2 * region_size, 1 * region_size),
        5: (2 * region_size, 2 * region_size),
        6: (3 * region_size, 2 * region_size),
    }

    def get_cube_face_for_testing(x, y):
        xx, yy = x // region_size, y // region_size
        if yy == 0:
            return 1
        elif yy == 1 and xx == 0:
            return 2
        elif yy == 1 and xx == 1:
            return 3
        elif yy == 1 and xx == 2:
            return 4
        elif yy == 2 and xx == 2:
            return 5
        else:
            return 6

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

10R5L5R10L4R5L5""",
            next_faces=next_faces,
            get_cube_face=get_cube_face_for_testing,
            origin_by_face=origin_by_face,
            region_size=region_size,
        )
        == 5031
    )
