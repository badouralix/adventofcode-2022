from tool.runners.python import SubmissionPy
from collections import defaultdict

rocks = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
]
width = 7


def move_right(rock, tetris):
    most_right = max(x for x, y in rock)
    if any(x + 1 in tetris[y] for x, y in rock):
        return rock
    new_most_right = min(most_right + 1, width - 1)
    offset = new_most_right - most_right
    return [(x + offset, y) for x, y in rock]


def move_left(rock, tetris):
    most_left = min(x for x, y in rock)
    if any(x - 1 in tetris[y] for x, y in rock):
        return rock
    new_most_left = max(most_left - 1, 0)
    offset = new_most_left - most_left
    return [(x + offset, y) for x, y in rock]


def move_down(rock, tetris):
    if any(x in tetris[y - 1] for x, y in rock):
        # blocked
        return [(x, y) for x, y in rock], True

    return [(x, y - 1) for x, y in rock], False


moves = {
    ">": move_right,
    "<": move_left,
}


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        tetris = defaultdict(set)
        tetris[0] = set(range(width))
        current_move = 0

        for i in range(2022):
            highest = max(y for y, points in tetris.items() if points)
            rock = [(x + 2, y + highest + 5) for x, y in rocks[i % len(rocks)]]
            blocked = False
            while not blocked:
                rock, blocked = move_down(rock, tetris)
                if blocked:
                    break
                rock = moves[s[current_move]](rock, tetris)
                current_move = (current_move + 1) % len(s)

            for x, y in rock:
                tetris[y].add(x)

        highest = max(y for y, points in tetris.items() if points)
        return highest


def test_th_ch():
    """
    Run `python -m pytest ./day-17/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
""".strip()
        )
        == 3068
    )
