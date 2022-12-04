from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        result = 0
        for line in s.split('\n'):
            a1, a2 = line.split(',')
            start1, end1 = [int(i) for i in a1.split('-')]
            start2, end2 = [int(i) for i in a2.split('-')]
            if (start2 >= start1 and end2 <= end1) or (start1 >= start2 and end1 <= end2):
                result += 1
        return result


def test_remicalixte():
    """
    Run `python -m pytest ./day-04/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
