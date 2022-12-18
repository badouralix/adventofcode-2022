from tool.runners.python import SubmissionPy
import ast


def is_right_order(left, right):
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            return 1
        elif left > right:
            return -1
        else:
            return 0
    elif isinstance(left, list) and isinstance(right, list):
        if len(left) == 0 and len(right) == 0:
            return 0
        elif len(left) == 0 and len(right) > 0:
            return 1
        elif len(left) > 0 and len(right) == 0:
            return -1
        is_ordered = is_right_order(left[0], right[0])
        if is_ordered == 0:
            return is_right_order(left[1:], right[1:])
        else:
            return is_ordered
    else:
        if isinstance(left, int):
            return is_right_order([left], right)
        else:
            return is_right_order(left, [right])


class YouyounSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        right_order_idx = set()
        for n, pair in enumerate(s.split("\n\n")):
            left, right = pair.split("\n")
            left = ast.literal_eval(left)
            right = ast.literal_eval(right)
            if is_right_order(left, right) == 1:
                right_order_idx.add(n + 1)
        return sum(right_order_idx)


def test_youyoun():
    """
    Run `python -m pytest ./day-13/part-1/youyoun.py` to test the submission.
    """
    assert (
            YouyounSubmission().run(
                """
    """.strip()
            )
            == None
    )
