from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        all_rucksacks = [set(x) for x in s.split("\n")]
        return sum(self.compute_priority(self.find_common_item(chunks)) for chunks in self.chunks(all_rucksacks, 3))

    @staticmethod
    def chunks(list, size):
        n = len(list)
        for idx in range(0, n, size):
            yield list[idx:idx+size]

    @staticmethod
    def find_common_item(chunks):
        intersection = set.intersection(*chunks)
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
