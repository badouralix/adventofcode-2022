from tool.runners.python import SubmissionPy


class Monkey:
    def __init__(self, items: list, operation, divisible: int, true_monkey: int, false_monkey: int) -> None:
        self.items = items
        self.operation = operation
        self.divisible = divisible
        self.true_monkey = true_monkey
        self.false_monkey = false_monkey
        self.inspected = 0

    def inspect(self):
        result = []
        self.inspected += len(self.items)
        for item in self.items:
            item = self.operation(item)
            item = item // 3
            if item % self.divisible == 0:
                result.append((item, self.true_monkey))
            else:
                result.append((item, self.false_monkey))
        
        self.items = []
            
        return result
            

def get_operation(op1, operator, op2):
    def operation_fun(old):
        if op1 == 'old':
            a = old
        else:
            a = int(op1)

        if op2 == 'old':
            b = old
        else:
            b = int(op2)

        return operator(a,b)
    return operation_fun
        

class RemicalixteSubmission(SubmissionPy):
    def run(self, s: str):
        monkeys = []
        for monkey in s.split('\n\n'):
            _, starting_items, operation, test, if_true, if_false = monkey.split('\n')
            items = []
            for item in starting_items.removeprefix('  Starting items: ').split(','):
                items.append(int(item.strip()))
            op1, fn, op2 = operation.removeprefix('  Operation: new = ').split()
            operator = None
            if fn == '+':
                operator = lambda a,b:a+b
            elif fn == '*':
                operator = lambda a,b: a*b


            divisible = int(test.removeprefix('  Test: divisible by '))
            true_monkey = int(if_true.removeprefix('    If true: throw to monkey '))
            false_monkey = int(if_false.removeprefix('    If false: throw to monkey '))

            monkeys.append(Monkey(items, get_operation(op1, operator, op2), divisible, true_monkey, false_monkey))

        for _ in range(20):
            for monkey in monkeys:
                dest = monkey.inspect()
                for (item, idx) in dest:
                    monkeys[idx].items.append(item)
        
        monkeys.sort(key=lambda monkey: monkey.inspected, reverse=True)
        top, next = monkeys[:2]

        return top.inspected * next.inspected
            



def test_remicalixte():
    """
    Run `python -m pytest ./day-11/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
