from tool.runners.python import SubmissionPy
import ast
from functools import cmp_to_key
from pprint import pprint

class DidipSubmission(SubmissionPy):
    @staticmethod
    def compare(left, right):
        if isinstance(left, int) and isinstance(right, int):
            if left < right:
                return -1

            if left == right:
                return 0
            
            return 1

        if isinstance(left, list) and isinstance(right, list):
            correct = 0
            i = 0
            while correct == 0:
                if len(left) == len(right) and len(left) == i:
                    return 0

                if len(left) <= i:
                    return -1

                if len(right) <= i:
                    return 1

                correct = DidipSubmission.compare(left[i], right[i])
                i += 1

            return correct

        if isinstance(left, list):
            return DidipSubmission.compare(left, [right])

        if isinstance(right, list):
            return DidipSubmission.compare([left], right)

        raise Exception()

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        signals = [[[2]], [[6]]]
        for line in s.splitlines():
            if line != '':
                signals.append(ast.literal_eval(line))

        signals_sorted = sorted(signals, key=cmp_to_key(lambda x, y: self.compare(x, y)))

        # pprint(signals_sorted)

        a = signals_sorted.index([[2]])
        b = signals_sorted.index([[6]])
        # print(a, b)
        return (a+1) * (b+1)



def test_didip():
    """
    Run `python -m pytest ./day-13/part-2/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
""".strip()
        )
        == 140
    )
