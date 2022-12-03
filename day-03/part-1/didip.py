from tool.runners.python import SubmissionPy

class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        score = 0
        for rucksack in s.splitlines():
            left, right = set(rucksack[:len(rucksack) // 2]), set(rucksack[len(rucksack) // 2:])
            (common,) =  left & right
            if ord(common) >= 97:
                score += ord(common) - 96
            else:
                score += ord(common) - 64 + 26

        return score



def test_didip():
    """
    Run `python -m pytest ./day-03/part-1/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
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
