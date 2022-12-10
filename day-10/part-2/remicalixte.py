from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        cycle = 1
        x = 1
        result = 0
        lines = [[]]
        for line in s.split('\n'):
            op = line.split()

            duration = 0
            add = 0
            if op[0] == 'noop':
                duration = 1
            else:
                add = int(op[1])
                duration = 2
            for _ in range(duration):
                if abs(((cycle-1) % 40) - x) < 2:
                    lines[-1].append('#')
                else:
                    lines[-1].append('.')
                if cycle % 40 == 0:
                    lines.append([])
                cycle += 1

            x += add


        return '\n'.join(''.join(line) for line in lines)
        
                




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
