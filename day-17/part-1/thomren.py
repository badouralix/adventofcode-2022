import itertools
from tool.runners.python import SubmissionPy

ROCKS = (
    [[1, 1, 1, 1]],
    [[0, 1, 0], [1, 1, 1], [0, 1, 0]],
    [[0, 0, 1], [0, 0, 1], [1, 1, 1]],
    [[1], [1], [1], [1]],
    [[1, 1], [1, 1]],
)
WIDTH = 7
N_ROCKS = 2022


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        jets = [1 if c == ">" else -1 for c in s.strip()]
        tower = set()
        height = 0
        rock_it = itertools.cycle(ROCKS)

        def get_move_it():
            jet_it = itertools.cycle(jets)
            while True:
                yield (next(jet_it), 0)
                yield (0, -1)

        move_it = get_move_it()

        for r in range(N_ROCKS):
            rock = next(rock_it)
            x = 2
            z = height + 3
            while True:
                dx, dz = next(move_it)
                chamber_bounds = (
                    x + dx >= 0 and x + dx + len(rock[0]) - 1 < WIDTH and z + dz >= 0
                )
                collision = False
                for i, line in enumerate(rock):
                    for j, b in enumerate(line):
                        if b == 1 and (x + dx + j, z + dz + len(rock) - 1 - i) in tower:
                            collision = True
                            break
                if chamber_bounds and not collision:
                    x += dx
                    z += dz
                elif dz == -1:
                    break

            for i, line in enumerate(rock):
                for j, b in enumerate(line):
                    if b == 1:
                        tower.add((x + j, z + len(rock) - 1 - i))
            height = max(z + len(rock), height)
            # if r < 10:
            #     print(r + 1, height)
            #     pprint_tower(tower)
            #     print()

        return height


def pprint_tower(tower):
    print(
        "\n".join(
            reversed(
                [
                    "".join(["#" if (j, i) in tower else "." for j in range(WIDTH)])
                    for i in range(25)
                ]
            )
        )
    )


def test_thomren():
    """
    Run `python -m pytest ./day-17/part-1/thomren.py` to test the submission.
    """
    assert ThomrenSubmission().run(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>") == 3068
