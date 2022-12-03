from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        score = 0
        for rucksack in s.splitlines():
            first_compartment = set(rucksack[: len(rucksack) // 2])
            second_compartment = set(rucksack[len(rucksack) // 2 :])
            for item in first_compartment.intersection(second_compartment):
                score += (
                    ord(item) - ord("a") + 1
                    if item.islower()
                    else ord(item) - ord("A") + 27
                )
        return score


def test_th_ch():
    """
    Run `python -m pytest ./day-03/part-1/th-ch.py` to test the submission.
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
        == 157
    )
