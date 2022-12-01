from tool.runners.python import SubmissionPy


class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        max_calories = 0
        current_calories = 0
        for line in s.split("\n"):
            if line == "":
                if current_calories > max_calories:
                    max_calories = current_calories
                current_calories = 0
            else:
                current_calories += int(line)
        return max_calories 


def test_silvestre():
    """
    Run `python -m pytest ./day-01/part-1\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
""".strip()
        )
        == None
    )
