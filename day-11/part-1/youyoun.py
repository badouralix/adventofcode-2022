import re
import operator

from tool.runners.python import SubmissionPy

OPS = {
    '*': operator.mul,
    '+': operator.add
}

class Monkey:
    def __init__(self, id, start_items, op_function, test_function, monkey_if_true, monkey_if_false):
        self.id = id
        self.items = start_items
        self.inspect = op_function
        self.test = test_function

        self.monkey_if_true = monkey_if_true
        self.monkey_if_false = monkey_if_false

    def __repr__(self):
        return f"Monkey {self.id}:\n Items: {self.items}\nMonkeyTrue: {self.monkey_if_true}\nMonkeyTrue: {self.monkey_if_false}"

    def throw(self):
        item = self.items.pop(0)
        # print(f"Monkey Inspects item {item}")
        importance = self.inspect(item) // 3
        # print(f"item importance after inspection and boredom {importance}")
        if self.test(importance):
            # print(f"Throw to monkey {self.monkey_if_true}")
            return importance, self.monkey_if_true
        else:
            # print(f"Throw to monkey {self.monkey_if_false}")
            return importance, self.monkey_if_false

    def receive(self, item):
        self.items.append(item)

def parse_input(monkey_data):
    regex_ = r"Monkey (\d+):\n\s+Starting items: ([\d,\s]*)\n\s+Operation: new = old ([*+]) (\d+|old)\n\s+Test: divisible by (\d+)\n\s+If true: throw to monkey (\d+)\n\s+If false: throw to monkey (\d+)"
    parsed = re.match(regex_, monkey_data)
    start_items = list(map(int, parsed.group(2).split(",")))
    inspect_func = lambda importance: OPS[parsed.group(3)](importance, importance if parsed.group(4)=='old' else int(parsed.group(4)))
    test_func = lambda importance: importance % int(parsed.group(5)) == 0
    return Monkey(int(parsed.group(1)), start_items, inspect_func, test_func, int(parsed.group(6)), int(parsed.group(7)))

class YouyounSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        n_rounds = 20
        monkeys = []
        inspected_counter = []
        for monkey_str in s.split("\n\n"):
            monkey = parse_input(monkey_str)
            monkeys.append(monkey)
            inspected_counter.append(0)
        for _ in range(n_rounds):
            for i in range(len(monkeys)):
                # print(f"Monkey {i}")
                inspected_counter[i] += len(monkeys[i].items)
                while len(monkeys[i].items) > 0:
                    item, throw_to_id = monkeys[i].throw()
                    monkeys[throw_to_id].receive(item)
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
