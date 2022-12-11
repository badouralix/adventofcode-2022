import re
import operator
import math

from tool.runners.python import SubmissionPy

OPS = {
    '*': operator.mul,
    '+': operator.add
}

class Monkey:
    def __init__(self, id, start_items, op_function, test_function, test_division, monkey_if_true, monkey_if_false):
        self.id = id
        self.items = start_items
        self.inspect = op_function
        self.test = test_function
        self.test_division = test_division

        self.monkey_if_true = monkey_if_true
        self.monkey_if_false = monkey_if_false

    def __repr__(self):
        return f"Monkey {self.id}:\n Items: {self.items}\nMonkeyTrue: {self.monkey_if_true}\nMonkeyTrue: {self.monkey_if_false}"

    def throw(self):
        importance = [self.inspect(item) for item in self.items]
        to_true = []
        to_false = []
        for imp in importance:
            if self.test(imp):
                to_true.append(imp)
            else:
                to_false.append(imp)
        self.items = []
        return to_true, self.monkey_if_true, to_false, self.monkey_if_false

    def receive(self, items):
        self.items.extend(items)

def parse_input(monkey_data):
    regex_ = r"Monkey (\d+):\n\s+Starting items: ([\d,\s]*)\n\s+Operation: new = old ([*+]) (\d+|old)\n\s+Test: divisible by (\d+)\n\s+If true: throw to monkey (\d+)\n\s+If false: throw to monkey (\d+)"
    parsed = re.match(regex_, monkey_data)
    start_items = list(map(int, parsed.group(2).split(",")))
    inspect_func = lambda importance: OPS[parsed.group(3)](importance, importance if parsed.group(4)=='old' else int(parsed.group(4)))
    test_func = lambda importance: importance % int(parsed.group(5)) == 0
    return Monkey(int(parsed.group(1)), start_items, inspect_func, test_func, int(parsed.group(5)), int(parsed.group(6)), int(parsed.group(7)))

class YouyounSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        n_rounds = 10000
        monkeys = []
        inspected_counter = []
        for monkey_str in s.split("\n\n"):
            monkey = parse_input(monkey_str)
            monkeys.append(monkey)
            inspected_counter.append(0)
        ppcm = math.lcm(*[m.test_division for m in monkeys])
        for _ in range(n_rounds):
            for i in range(len(monkeys)):
                inspected_counter[i] += len(monkeys[i].items)
                items_true, monkey_id_true, items_false, monkey_id_false = monkeys[i].throw()
                monkeys[monkey_id_true].receive([item % ppcm for item in items_true])
                monkeys[monkey_id_false].receive([item % ppcm for item in items_false])
        inspected_counter = sorted(inspected_counter)
        return inspected_counter[-1] * inspected_counter[-2]


def test_youyoun():
    """
    Run `python -m pytest ./day-11/part-1/youyoun.py` to test the submission.
    """
    assert (
            YouyounSubmission().run(
                """
    """.strip()
            )
            == None
    )
