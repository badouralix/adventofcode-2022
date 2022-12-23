from collections import deque
from tool.runners.python import SubmissionPy


class MonkeyIsHuman(Exception):
    pass


def resolve_monkey(monkeys, monkey):
    if monkey == "humn":
        raise MonkeyIsHuman()

    if isinstance(monkeys[monkey], int):
        return monkeys[monkey]

    m1, op, m2 = monkeys[monkey]
    operation = (
        lambda a, b: a + b
        if op == "+"
        else (a - b if op == "-" else (a * b if op == "*" else a // b))
    )
    return operation(resolve_monkey(monkeys, m1), resolve_monkey(monkeys, m2))


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        monkeys = {}
        monkeys_to_match = None
        for line in s.splitlines():
            monkey, operation = line.split(": ")
            if operation.isnumeric():
                monkeys[monkey] = int(operation)
            else:
                m1, op, m2 = operation.split(" ")
                if monkey == "root":
                    monkeys_to_match = (m1, m2)
                else:
                    monkeys[monkey] = (m1, op, m2)

        m1, m2 = monkeys_to_match
        monkey_to_human = None
        try:
            val = resolve_monkey(monkeys, m1)
            monkey_to_human = m2
        except MonkeyIsHuman:
            val = resolve_monkey(monkeys, m2)
            monkey_to_human = m1

        queue = deque()
        queue.append(monkey_to_human)
        monkey_expected_values = {monkey_to_human: val}
        while queue:
            monkey = queue.popleft()
            if monkey == "humn":
                return monkey_expected_values[monkey]

            if isinstance(monkeys[monkey], int):
                monkey_expected_values[monkey] = monkeys[monkey]
                continue

            expected_value = monkey_expected_values[monkey]
            m1, op, m2 = monkeys[monkey]
            try:
                val = resolve_monkey(monkeys, m1)
                monkey_to_resolve = m2
                # expected_value = m1 OP m2
                if op == "+":
                    expected_value -= val
                elif op == "-":
                    expected_value = val - expected_value
                elif op == "*":
                    expected_value //= val
                else:
                    expected_value = m1 // expected_value
            except MonkeyIsHuman:
                val = resolve_monkey(monkeys, m2)
                monkey_to_resolve = m1
                # expected_value = m1 OP m2
                if op == "+":
                    expected_value -= val
                elif op == "-":
                    expected_value += val
                elif op == "*":
                    expected_value //= val
                else:
                    expected_value *= val

            monkey_expected_values[monkey_to_resolve] = expected_value
            queue.append(monkey_to_resolve)


def test_th_ch():
    """
    Run `python -m pytest ./day-21/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
""".strip()
        )
        == 301
    )
