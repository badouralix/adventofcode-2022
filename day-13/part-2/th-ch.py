from tool.runners.python import SubmissionPy

import ast
from functools import cmp_to_key
from importlib import import_module

part1 = import_module("day-13.part-1.th-ch")


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        packets = [ast.literal_eval(line) for line in s.splitlines() if line] + [
            [[2]],
            [[6]],
        ]
        sorted_packets = sorted(
            packets, key=cmp_to_key(part1.are_in_right_order), reverse=True
        )
        return (1 + sorted_packets.index([[2]])) * (1 + sorted_packets.index([[6]]))


def test_th_ch():
    """
    Run `python -m pytest ./day-13/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
""".strip()
        )
        == 140
    )
