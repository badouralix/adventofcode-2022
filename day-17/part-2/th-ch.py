from tool.runners.python import SubmissionPy
from collections import defaultdict
from importlib import import_module

part1 = import_module("day-17.part-1.th-ch")

rocks = part1.rocks
width = part1.width
move_right = part1.move_right
move_left = part1.move_left
move_down = part1.move_down
moves = part1.moves


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        tetris = defaultdict(set)
        tetris[0] = set(range(width))
        current_move = 0
        cycles = defaultdict(dict)  # Detect cycles
        bonus_height_from_cycles = 0

        nb_rocks = 1000000000000
        i = 0

        while i < nb_rocks:
            highest = max(y for y, points in tetris.items() if points)
            rock = [(x + 2, y + highest + 5) for x, y in rocks[i % len(rocks)]]
            blocked = False
            while not blocked:
                rock, blocked = move_down(rock, tetris)
                if blocked:
                    break
                rock = moves[s[current_move]](rock, tetris)
                current_move = (current_move + 1) % len(s)

            # Try to detect a cycle
            item = (current_move, i % len(rocks), str(list(x for x, _ in rock)))
            highest = max(y for y, points in tetris.items() if points)
            cycles[item][highest] = i
            if len(cycles[item]) >= 3:
                heights = sorted(cycles[item].keys())
                if [tetris[y] for y in range(heights[0], heights[1])] == [
                    tetris[y] for y in range(heights[1], heights[2])
                ]:
                    # cycle detected! It's between heights[0] and heights[1]
                    cycle_height = heights[1] - heights[0]
                    nb_rocks_in_cycle = (
                        cycles[item][heights[1]] - cycles[item][heights[0]]
                    )
                    nb_cycles = (nb_rocks - i) // nb_rocks_in_cycle
                    # Jump all cycles to iterate faster
                    i += nb_cycles * nb_rocks_in_cycle
                    bonus_height_from_cycles += nb_cycles * cycle_height

            i += 1

            for x, y in rock:
                tetris[y].add(x)

        highest = max(y for y, points in tetris.items() if points)
        return highest + bonus_height_from_cycles


def test_th_ch():
    """
    Run `python -m pytest ./day-17/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
""".strip()
        )
        == 1514285714288
    )
