from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        cycle = 1
        next_signal = 20
        x = 1
        result = 0
        for line in s.split('\n'):
            op = line.split()

            duration = 0
            add = 0
            if op[0] == 'noop':
                duration = 1
            else:
                add = int(op[1])
                duration = 2
            while duration > 0:
                cycle += 1
                duration -= 1
                if duration == 0:
                    x += add
                if cycle == next_signal:
                    result += cycle*x
                    next_signal += 40
        
        return result
        
                




def test_remicalixte():
    """
    Run `python -m pytest ./day-10/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
