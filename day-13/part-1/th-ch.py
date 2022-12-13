from tool.runners.python import SubmissionPy

import ast


def are_in_right_order(left, right):
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            return 1
        elif left > right:
            return -1
        else:
            return 0

    if isinstance(left, list) and isinstance(right, list):
        if not left and not right:
            return 0
        if not left and right:
            return 1
        if not right and left:
            return -1

        are_ordered = are_in_right_order(left[0], right[0])
        if are_ordered != 0:
            return are_ordered
        else:
            return are_in_right_order(left[1:], right[1:])

    # One is an integer, the other a list
    if isinstance(left, int):
        return are_in_right_order([left], right)
    else:
        return are_in_right_order(left, [right])


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        total = 0
        for i, pair in enumerate(s.split("\n\n")):
            left, right = map(ast.literal_eval, pair.splitlines())
            is_in_right_order = are_in_right_order(left, right)
            if is_in_right_order == 1:
                total += i + 1
        return total


def test_th_ch():
    """
    Run `python -m pytest ./day-13/part-1/th-ch.py` to test the submission.
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
        == 13
    )
