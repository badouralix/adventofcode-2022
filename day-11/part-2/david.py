from dataclasses import dataclass
from tool.runners.python import SubmissionPy

from typing import List, Callable, Tuple

@dataclass
class Monkey:
    id: int
    items: List[int]
    operation: Callable[[int], int]
    divisor: int
    if_true: int
    if_false: int

    inspections: int = 0

    @classmethod
    def from_str(cls, id: int, s: str):
        lines = s.split("\n")
        starting_items = [int(x) for x in lines[1][len("  Starting items: "):].split(", ")]
        operation = cls.parse_operation(lines[2])
        divisor = int(lines[3].split(" ")[-1])
        if_true = int(lines[4].split(" ")[-1])
        if_false = int(lines[5].split(" ")[-1])

        return Monkey(id, starting_items, operation, divisor, if_true, if_false)

    @staticmethod
    def parse_operation(s: str) -> Callable[[int], int]:
        op_str = s.split(" = ")[1]
        return lambda old: eval(op_str)

    def target_monkey(self, condition: bool):
        if condition:
            return self.if_true
        return self.if_false


class DavidSubmission(SubmissionPy):
    ROUNDS = 10_000

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        monkeys = [Monkey.from_str(idx, block) for idx, block in enumerate(s.split("\n\n"))]

        least_common_multiple = 1
        for monkey in monkeys:
            # assuming all the divisors are prime...
            least_common_multiple *= monkey.divisor

        for _ in range(self.ROUNDS):
            for monkey in monkeys:
                while monkey.items:
                    monkey.inspections += 1
                    item = monkey.items.pop(0)
                    level = monkey.operation(item) % least_common_multiple
                    next_monkey = monkey.target_monkey(level % monkey.divisor == 0)
                    monkeys[next_monkey].items.append(level)
            
        inspections = sorted((monkey.inspections for monkey in monkeys), reverse=True)
        return inspections[0] * inspections[1]