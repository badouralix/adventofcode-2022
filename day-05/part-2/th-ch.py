from collections import deque
from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        starting_stacks, instructions = s.split("\n\n")
        starting_stacks = starting_stacks.split("\n")
        starting_stacks.reverse()
        nb_crates = len(starting_stacks[0].split())
        crates = [deque() for _ in range(nb_crates)]
        for line in starting_stacks[1:]:
            # each crate = 4 chars
            nb_crates_in_line = len(line) // 4 + 1
            for i in range(nb_crates_in_line):
                item = line[4 * i : 4 * (i + 1)].strip()
                if item:
                    crates[i].appendleft(item.replace("[", "").replace("]", ""))

        for instruction in instructions.splitlines():
            split_instruction = instruction.split()
            move, from_crate, to_crate = (
                int(split_instruction[1]),
                int(split_instruction[3]),
                int(split_instruction[5]),
            )
            to_move = [crates[from_crate - 1].popleft() for _ in range(move)]
            to_move.reverse()
            crates[to_crate - 1].extendleft(to_move)

        return "".join(crate.popleft() for crate in crates)


def test_th_ch():
    """
    Run `python -m pytest ./day-05/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"""
        )
        == "MCD"
    )
