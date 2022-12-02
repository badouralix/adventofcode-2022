from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        elves = []
        calories = s.split('\n')
        elf = 0
        for calorie in calories:
            if calorie == '':
                elves.append(elf)
                elf = 0
                continue
            elf += int(calorie)
        elves.sort(reverse=True)
        return sum(elves[:3])

def test_remicalixte():
    """
    Run `python -m pytest ./day-01/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
