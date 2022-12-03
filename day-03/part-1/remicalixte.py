from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        priorities = 0
        for line in s.split('\n'):
            comp1, comp2 = set(line[:len(line)//2]), set(line[len(line)//2:])
            intersec = comp1 & comp2
            common = ord(intersec.pop())


            priority = 0
            if ord('a') <= common and common <= ord('z'):
                priority = common - ord('a') + 1
            elif ord('A') <= common and common <= ord('Z'):
                priority = common - ord('A') + 27

            priorities += priority

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
