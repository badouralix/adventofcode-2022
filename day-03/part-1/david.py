from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        rucksacks = s.split("\n")
        return sum(self.compute_priority(self.find_common_item(rucksack)) for rucksack in rucksacks)

    @staticmethod
    def find_common_item(rucksack: str) -> str:
        middle = len(rucksack) >> 1
        left = set(rucksack[:middle])
        right = set(rucksack[middle:])
        intersection = left & right
        assert len(intersection) == 1
        return intersection.pop()

    @staticmethod
    def compute_priority(item: str) -> int:
        priority = ord(item.lower()) - ord("a") + 1
        if not item.islower():
            priority += 26
        return priority




def test_david():
    """
    Run `python -m pytest ./day-03/part-1/david.py` to test the submission.
    """
    assert (
        DavidSubmission().run(
            """
""".strip()
        )
        == None
    )
