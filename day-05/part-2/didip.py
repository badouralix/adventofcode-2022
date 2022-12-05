from tool.runners.python import SubmissionPy
import re

class DidipSubmission:
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        n_stacks = 9
        stacks = [[] for _ in range(n_stacks)]
        initial, orders = s.split('\n\n')

        for line in initial.splitlines():
            if line[1] != '1':
                for i in range(n_stacks):
                    if line[4 * i + 1] != ' ':
                        stacks[i] = [line[4 * i + 1]] + stacks[i]

        for order in orders.splitlines():
            m = re.match(r'move (?P<amount>\d+) from (?P<stack_from>\d) to (?P<stack_to>\d)', order)
            i_from = int(m.group('stack_from')) - 1
            i_to = int(m.group('stack_to')) - 1
            amount = int(m.group('amount'))

            stacks[i_to].extend(stacks[i_from][-amount:])
            stacks[i_from] = stacks[i_from][:-amount]

        return ''.join([stacks[i][-1] for i in range(n_stacks)])

