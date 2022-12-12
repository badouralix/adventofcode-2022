from tool.runners.python import SubmissionPy


class DidipSubmission(SubmissionPy):
    @staticmethod
    def update_crt(crt, line, position, reg_x):
        if reg_x - 1 == position or reg_x == position or reg_x + 1 == position:
            crt[line][position] = '#'


    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        reg_x = 1
        cycle = 1

        crt = [['.' for _ in range(40)] for _ in range(6)]

        for instruction in s.splitlines():
            self.update_crt(crt, (cycle - 1) // 40, (cycle - 1) % 40, reg_x)

            if instruction[:4] == 'addx':
                # Also update on second cycle
                self.update_crt(crt, cycle // 40, cycle % 40, reg_x)

                value = int(instruction.split()[1])
                reg_x += value
                cycle += 2

            else:
                cycle += 1

        return '\n'.join(map(lambda e: ''.join(e), crt))



def test_didip():
    """
    Run `python -m pytest ./day-10/part-2/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """
""".strip()
        )
        == None
    )
