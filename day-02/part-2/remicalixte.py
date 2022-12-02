from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        score = 0
        for round in s.split('\n'):
            raw = round.split(' ')
            op = ord(raw[0]) - 65
            self = ord(raw[1]) - 88

            score += self * 3

            score += (op + self -1) % 3 + 1


        return score




def test_remicalixte():
    """
    Run `python -m pytest ./day-02/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
