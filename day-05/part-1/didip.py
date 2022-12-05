from tool.runners.python import SubmissionPy
import re

class DidipSubmission(SubmissionPy):
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
                    if len(line) > 4 * i + 1 and line[4 * i + 1] != ' ':
                        stacks[i] = [line[4 * i + 1]] + stacks[i]

        for order in orders.splitlines():
            order_inst = order.split(' ')
            i_from, i_to, amount = int(order_inst[3]) - 1, int(order_inst[5]) - 1, int(order_inst[1])

            stacks[i_to].extend(stacks[i_from][-amount:][::-1])
            stacks[i_from] = stacks[i_from][:-amount]

        return ''.join([stacks[i][-1] for i in range(n_stacks)])

