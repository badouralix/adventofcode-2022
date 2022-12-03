from tool.runners.python import SubmissionPy


def find_common(part1, part2, part3):
    for x in part1:
        if x in part2 and x in part3:
            return x


priorities = {x: i + 1 for i, x in enumerate('abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ')}


class BebertSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        res = 0
        group = []
        for line in s.splitlines():
            group.append(line)
            if len(group) == 3:
                res += priorities[find_common(*group)]
                group = []
        return res


def test_bebert():
    """
    Run `python -m pytest ./day-03/part-2/bebert.py` to test the submission.
    """
    assert (
            BebertSubmission().run(
                """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
""".strip()
            )
            == 70
    )
