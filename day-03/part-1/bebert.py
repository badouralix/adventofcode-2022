from tool.runners.python import SubmissionPy


def find_common(part1, part2):
    for x in part1:
        if x in part2:
            return x


priorities = {x: i + 1 for i, x in enumerate('abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ')}


class BebertSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        res = 0
        for line in s.splitlines():
            middle = len(line) // 2
            part1 = line[:middle]
            part2 = line[middle:]
            common = find_common(part1, part2)
            res += priorities[common]
        return res


def test_bebert():
    """
    Run `python -m pytest ./day-03/part-1/bebert.py` to test the submission.
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
            == 157
    )
