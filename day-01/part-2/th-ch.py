from tool.runners.python import SubmissionPy

import bisect


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        top = 3
        top_max_calories = []
        current_calories = 0
        for line in s.splitlines() + [None]:
            if not line:
                bisect.insort(top_max_calories, current_calories)
                top_max_calories = top_max_calories[-top:]
                current_calories = 0
            else:
                current_calories += int(line)
        return sum(top_max_calories)


def test_th_ch():
    """
    Run `python -m pytest ./day-01/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
1000
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
        == 45000
    )
