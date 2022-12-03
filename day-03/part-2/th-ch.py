from tool.runners.python import SubmissionPy

import re


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        score = 0
        for group in re.findall("((?:[^\n]+\n?){1,3})", s):
            badge = set.intersection(*[set(elf) for elf in group.splitlines()])
            for item in badge:
                score += (
                    ord(item) - ord("a") + 1
                    if item.islower()
                    else ord(item) - ord("A") + 27
                )
        return score


def test_th_ch():
    """
    Run `python -m pytest ./day-03/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
""".strip()
        )
        == 70
    )
