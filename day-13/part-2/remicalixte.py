from tool.runners.python import SubmissionPy
import functools

class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        d1 = [[2]]
        d2 = [[6]]
        packets = [d1, d2]
        for line in s.split('\n\n'):
            left, right = line.split('\n')
            left = eval(left)
            right = eval(right)

            packets.append(left)
            packets.append(right)

        packets.sort(key=functools.cmp_to_key(compare))


        i = 0
        j = 0
        for k, p in enumerate(packets):
            if p == d1:
                i = k+1
            if p == d2:
                j = k+1

        return i*j


def compare(left, right):
    if isinstance(left, int) and isinstance(right, int):
        return left - right
    elif isinstance(left, list) and isinstance(right, list):
        for i in range(min(len(left), len(right))):
            r = compare(left[i], right[i])
            if r != 0:
                return r
        return len(left) - len(right)
    elif isinstance(left, int):
        return compare([left], right)
    else:
        return compare(left, [right])


def test_remicalixte():
    """
    Run `python -m pytest ./day-13/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
