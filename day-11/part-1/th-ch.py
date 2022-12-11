from tool.runners.python import SubmissionPy

from collections import deque


class Monkey:
    def __init__(
        self,
        monkey_id,
        items,
        operation,
        divisible_by,
        next_monkey_if_true,
        next_monkey_if_false,
    ):
        self.monkey_id = monkey_id
        self.items = deque(items)
        self.operation = operation
        self.divisible_by = divisible_by
        self.next_monkey_if_true = next_monkey_if_true
        self.next_monkey_if_false = next_monkey_if_false
        self.inspected = 0

    def inspect_item(self):
        self.inspected += 1
        old = self.items.popleft()
        item = eval(self.operation) // 3
        if item % self.divisible_by == 0:
            return item, self.next_monkey_if_true
        else:
            return item, self.next_monkey_if_false

    def add_item(self, item):
        self.items.append(item)

    def __str__(self):
        return "Monkey {}: {}".format(self.monkey_id, self.items)


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        monkeys = []
        for monkey_input in s.split("\n\n"):
            for i, line in enumerate(monkey_input.splitlines()):
                line = line.strip()
                if i == 0:
                    monkey_id = int(line.replace("Monkey ", "")[:-1])
                if i == 1:
                    starting_items = [
                        int(item)
                        for item in line.replace("Starting items: ", "").split(", ")
                    ]
                elif i == 2:
                    operation = line.replace("Operation: new = ", "")
                elif i == 3:
                    divisible_by = int(line.replace("Test: divisible by ", ""))
                elif i == 4:
                    next_monkey_if_true = int(
                        line.replace("If true: throw to monkey ", "")
                    )
                elif i == 5:
                    next_monkey_if_false = int(
                        line.replace("If false: throw to monkey ", "")
                    )
            monkeys.append(
                Monkey(
                    monkey_id,
                    starting_items,
                    operation,
                    divisible_by,
                    next_monkey_if_true,
                    next_monkey_if_false,
                )
            )

        for monkey_round in range(20):
            for monkey in monkeys:
                while monkey.items:
                    item, next_monkey_id = monkey.inspect_item()
                    monkeys[next_monkey_id].add_item(item)

        inspected = sorted(monkey.inspected for monkey in monkeys)
        return inspected[-1] * inspected[-2]


def test_th_ch():
    """
    Run `python -m pytest ./day-11/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
Monkey 0:
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
        == 10605
    )
