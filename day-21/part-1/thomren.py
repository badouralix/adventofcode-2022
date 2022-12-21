from graphlib import TopologicalSorter
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        monkeys = {}
        for line in s.splitlines():
            tokens = line.split()
            monkey = tokens[0][:-1]
            if len(tokens) == 2:
                monkeys[monkey] = int(tokens[1])
            else:
                monkeys[monkey] = tokens[1:]

        def compute(x):
            if type(monkeys[x]) == int:
                return monkeys[x]
            a, op, b = monkeys[x]
            if op == "+":
                return compute(a) + compute(b)
            elif op == "-":
                return compute(a) - compute(b)
            elif op == "*":
                return compute(a) * compute(b)
            elif op == "/":
                return compute(a) // compute(b)

        return compute("root")


def test_thomren():
    """
    Run `python -m pytest ./day-21/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
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
