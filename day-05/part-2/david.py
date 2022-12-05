from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    COUNT_STACKS = 9
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        initial_state_input, moves_input = s.split("\n\n")
        stacks = [None] + [[] for _ in range(self.COUNT_STACKS)]
        for line in initial_state_input.split("\n")[:-1]:
            stack_idx = 1
            while (pos := 4*stack_idx-3) < len(line):
                if line[pos].isalpha():
                    stacks[stack_idx].append(line[pos])
                stack_idx += 1
                
        for move in moves_input.split("\n"):
            _, count, _, source, _, target = move.split(" ")
            count, source, target = int(count), int(source), int(target)
            
            to_move = stacks[source][0:count]
            stacks[target] = to_move + stacks[target]
            stacks[source] = stacks[source][count:]

        return "".join(stack[0] for stack in stacks[1:])



def test_david():
    """
    Run `python -m pytest ./day-05/part-1/david.py` to test the submission.
    """
    assert (
        DavidSubmission().run(
            """
""".strip()
        )
        == None
    )
