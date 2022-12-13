from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        result = 0
        for i, line in enumerate(s.split('\n\n')):
            left, right = line.split('\n')

            left = eval(left)
            right = eval(right)

            if compare(left, right) < 0:
                result += i+1
        return result


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
