from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        ready = set()
        monkeys = {}
        reversed_graph = {}
        for line in s.splitlines():
            tokens = line.split()
            monkey = tokens[0][:-1]
            if len(tokens) == 2:
                ready.add(monkey)
                monkeys[monkey] = int(tokens[1])
            else:
                monkeys[monkey] = tokens[1:]
                # we assume that several monkeys cannot be waiting for the same other monkey
                reversed_graph[tokens[1]] = monkey
                reversed_graph[tokens[3]] = monkey

        def compute(x):
            if x == "humn":
                raise ValueError
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

        def solve(x):
            parent = reversed_graph[x]
            a, op, b = monkeys[parent]
            if parent == "root":
                return compute(a) if b == x else compute(b)
            if x == a:
                if op == "+":
                    return solve(parent) - compute(b)
                elif op == "-":
                    return solve(parent) + compute(b)
                elif op == "*":
                    return solve(parent) // compute(b)
                elif op == "/":
                    return solve(parent) * compute(b)
            elif x == b:
                if op == "+":
                    return solve(parent) - compute(a)
                elif op == "-":
                    return compute(a) - solve(parent)
                elif op == "*":
                    return solve(parent) // compute(a)
                elif op == "/":
                    return compute(a) // solve(parent)

        return solve("humn")


def test_thomren():
    """
    Run `python -m pytest ./day-21/part-2/thomren.py` to test the submission.
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
        == 301
    )
