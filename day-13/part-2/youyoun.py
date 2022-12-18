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


def merge_sort(list_):
    if len(list_) <= 1:
        return list_

    left = list_[:len(list_) // 2]
    right = list_[len(list_) // 2:]
    left = merge_sort(left)
    right = merge_sort(right)
    return merge(left, right)


def merge(left, right):
    # print("start", left, right)
    result = []
    while len(left) != 0 and len(right) != 0:
        if is_right_order(left[0], right[0]) == 1:
            result.append(left.pop(0))
        else:
            result.append(right.pop(0))

    if len(left) == 0:
        result.extend(right)
    else:
        result.extend(left)
    # print("end", left, right, result)
    return result


class YouyounSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        list_ = [ast.literal_eval(x) for x in s.replace("\n\n", "\n").split("\n")] + [ [[2]], [[6]] ]
        sorted_ = merge_sort(list_)
        idx_2, idx_6 = 0, 0
        for i, e in enumerate(sorted_):
            if str(e) == "[[2]]":
                idx_2 = i + 1
            elif str(e) == "[[6]]":
                idx_6 = i + 1
        return idx_2 * idx_6


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
