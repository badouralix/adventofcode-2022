from tool.runners.python import SubmissionPy

ROCKS = (
    [[1, 1, 1, 1]],
    [[0, 1, 0], [1, 1, 1], [0, 1, 0]],
    [[0, 0, 1], [0, 0, 1], [1, 1, 1]],
    [[1], [1], [1], [1]],
    [[1, 1], [1, 1]],
)
WIDTH = 7
N_ROCKS = 1000000000000
CYCLE_DETECTION_HEIGHT = 10


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        jets = [1 if c == ">" else -1 for c in s.strip()]
        tower = set()
        height = 0
        seen = {}
        jet_idx = 0
        rock_idx = 0

        r = 0
        while r < N_ROCKS:
            r += 1
            rock = ROCKS[rock_idx]
            rock_idx = (rock_idx + 1) % len(ROCKS)
            x = 2
            z = height + 3
            dx, dz = 0, -1
            while True:
                if dx == 0:
                    dx = jets[jet_idx]
                    jet_idx = (jet_idx + 1) % len(jets)
                    dz = 0
                else:
                    dx = 0
                    dz = -1

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
            state = (
                jet_idx,
                rock_idx,
                pprint_tower(tower, height - CYCLE_DETECTION_HEIGHT, height + 1),
            )
            if state in seen:
                old_height = height
                cycle_len = r - seen[state][0]
                cycle_height = height - seen[state][1]
                n_cycles = (N_ROCKS - r) // cycle_len
                height += cycle_height * n_cycles
                r += cycle_len * n_cycles
                for i in range(WIDTH):
                    for z in range(old_height - CYCLE_DETECTION_HEIGHT, old_height + 1):
                        if (i, z) in tower:
                            tower.add((i, z + cycle_height * n_cycles))
            seen[state] = (r, height)

        return height


def pprint_tower(tower, from_z, to_z):
    return "\n".join(
        reversed(
            [
                "".join(["#" if (j, i) in tower else "." for j in range(WIDTH)])
                for i in range(from_z, to_z)
            ]
        )
    )


def test_thomren():
    """
    Run `python -m pytest ./day-17/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")
        == 1514285714288
    )
