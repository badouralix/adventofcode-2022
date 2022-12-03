from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        priorities = 0
        elves = []
        for i, line in enumerate(s.split('\n')):
            elves.append(line)
            if i % 3 == 2:
                elf1, elf2, elf3 = [set(elf) for elf in elves]
                intersec = elf1.intersection(elf2).intersection(elf3)
                common = ord(intersec.pop())
                priority = 0
                if ord('a') <= common and common <= ord('z'):
                    priority = common - ord('a') + 1
                elif ord('A') <= common and common <= ord('Z'):
                    priority = common - ord('A') + 27

                priorities += priority
                elves = []

        return priorities







def test_remicalixte():
    """
    Run `python -m pytest ./day-03/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
