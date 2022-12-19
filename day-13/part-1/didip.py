from tool.runners.python import SubmissionPy
import ast

class DidipSubmission(SubmissionPy):
    @staticmethod
    def compare(left, right):
        if isinstance(left, int) and isinstance(right, int):
            if left < right:
                return True

            if left == right:
                return None
            
            return False

        if isinstance(left, list) and isinstance(right, list):
            correct = None
            i = 0
            while correct == None:
                if len(left) == len(right) and len(left) == i:
                    return None

                if len(left) <= i:
                    return True

                if len(right) <= i:
                    return False

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
        c = 0
        for i, pair in enumerate(s.split('\n\n')):
            left, right = pair.strip().split('\n')
            left = ast.literal_eval(left)
            right = ast.literal_eval(right)
            if self.compare(left, right):
                c += i + 1

        return c



def test_didip():
    """
    Run `python -m pytest ./day-13/part-1/didip.py` to test the submission.
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

[[8,[[7]]]]
[[[[8]]]]

""".strip()
        )
        == 13
    )
