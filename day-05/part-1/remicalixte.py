from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s: str):
        stack_lines = []
        number_line = ''
        move_lines = []
        for line in s.split('\n'):
            if line.startswith('[') or line.startswith('   '):
                stack_lines.append(line)
            elif line.strip().startswith('1'):
                number_line = line
            elif line.startswith('m'):
                move_lines.append(line)

        stack_number = int(number_line.split()[-1])
        stacks = [[] for i in range(stack_number)]
        for line in reversed(stack_lines):
            for i in range (stack_number):
                stuff = line[1+i*4]
                if stuff != ' ':
                    stacks[i].append(line[1+i*4])

        for line in move_lines:
            line = line[5:]
            cnt, move = line.split('from')
            start, end = move.split('to')
            cnt = int(cnt.strip())
            start = int(start.strip())
            end = int(end.strip())


            for _ in range(cnt):
                stacks[end-1].append(stacks[start-1].pop())


        return ''.join(stack[-1] for stack in stacks)








def test_remicalixte():
    """
    Run `python -m pytest ./day-05/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
