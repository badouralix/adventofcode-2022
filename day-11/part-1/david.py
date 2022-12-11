from dataclasses import dataclass
from tool.runners.python import SubmissionPy

from typing import List, Callable, Tuple

@dataclass
class Monkey:
    id: int
    items: List[int]
    operation: Callable[[int], int]
    diviser: int
    if_true: int
    if_false: int

    inspections: int = 0

    @classmethod
    def from_str(cls, id: int, s: str):
        lines = s.split("\n")
        starting_items = [int(x) for x in lines[1][len("  Starting items: "):].split(", ")]
        operation = cls.parse_operation(lines[2])
        diviser = int(lines[3].split(" ")[-1])
        if_true = int(lines[4].split(" ")[-1])
        if_false = int(lines[5].split(" ")[-1])
        return Monkey(id, starting_items, operation, diviser, if_true, if_false)

    @staticmethod
    def parse_operation(s: str) -> Callable[[int], int]:
        op_str = s.split(" = ")[1]
        return lambda old: eval(op_str)

    def target_monkey(self, condition: bool):
        if condition:
            return self.if_true
        return self.if_false

class DavidSubmission(SubmissionPy):
    ROUNDS = 20

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        monkeys = [Monkey.from_str(idx, block) for idx, block in enumerate(s.split("\n\n"))]

        for _ in range(self.ROUNDS):
            for monkey in monkeys:
                while monkey.items:
                    monkey.inspections += 1
                    item = monkey.items.pop(0)
                    level = monkey.operation(item)
                    level = int(level/3)
                    monkeys[monkey.target_monkey(level % monkey.diviser == 0)].items.append(level)

        inspections = sorted((monkey.inspections for monkey in monkeys), reverse=True)
        return inspections[0] * inspections[1]