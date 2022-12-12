from tool.runners.python import SubmissionPy
from math import lcm

class DidipSubmission(SubmissionPy):
    @staticmethod
    def make_operation(operation_raw):
        if operation_raw[1] == '+':
            if operation_raw[2] == 'old':
                return lambda x: x + x
            else:
                return lambda x: x + int(operation_raw[2])

        else:
            if operation_raw[2] == 'old':
                return lambda x: x * x
            else:
                return lambda x: x * int(operation_raw[2])


    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        lines_per_monkey = 7
        n_rounds = 10000
        lines = s.splitlines()
        n_monkeys = (len(lines) + 1) // lines_per_monkey
        monkeys = {}
        for monkey in range(0, len(lines), lines_per_monkey):
            start_items = list(map(int, lines[monkey + 1].split(':')[1].split(',')))
            operation_raw = lines[monkey + 2].split('=')[1].split()
            divisibility_test = int(lines[monkey + 3].split()[-1])
            if_true = int(lines[monkey + 4].split()[-1])
            if_false = int(lines[monkey + 5].split()[-1])

            monkeys[monkey // lines_per_monkey] = {
                'objects': start_items,
                'operation': self.make_operation(operation_raw),
                'divisibility': divisibility_test,
                'action': {True: if_true, False: if_false},
                'inspection': 0
            }


        int_max = lcm(*[monkey['divisibility'] for monkey in monkeys.values()])

        for n_round in range(n_rounds):
            # if n_round % 100 == 99:
            #     print(n_round)
            #     print(monkeys[0]['objects'])
            for i in range(n_monkeys):
                monkey = monkeys[i]
                for item in monkey['objects']:
                    item_worry = monkey['operation'](item) % int_max
                    
                    new_monkey = monkey['action'][item_worry % monkey['divisibility'] == 0]
                    monkeys[new_monkey]['objects'].append(item_worry)
                
                monkey['inspection'] += len(monkey['objects'])
                monkey['objects'] = []

        inspections = sorted([monkeys[monkey]['inspection'] for monkey in range(n_monkeys)])
        return inspections[-1] * inspections[-2]


def test_didip():
    """
    Run `python -m pytest ./day-11/part-2/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
""".strip()
        )
        == 2713310158
    )
