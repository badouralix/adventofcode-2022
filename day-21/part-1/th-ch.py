from collections import deque
from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        number_by_monkey = {}
        queue = deque()
        for line in s.splitlines():
            monkey, operation = line.split(": ")
            if operation.isnumeric():
                number_by_monkey[monkey] = int(operation)
            else:
                m1, op, m2 = operation.split(" ")
                queue.append((monkey, m1, op, m2))

        while queue:
            monkey, m1, op, m2 = queue.popleft()
            if m1 in number_by_monkey and m2 in number_by_monkey:
                operation = (
                    lambda a, b: a + b
                    if op == "+"
                    else (a - b if op == "-" else (a * b if op == "*" else a // b))
                )
                number_by_monkey[monkey] = operation(
                    number_by_monkey[m1], number_by_monkey[m2]
                )
                if monkey == "root":
                    return number_by_monkey[monkey]
            else:
                queue.append((monkey, m1, op, m2))


def test_th_ch():
    """
    Run `python -m pytest ./day-21/part-1/th-ch.py` to test the submission.
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
        == 152
    )
