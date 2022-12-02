from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        score = 0
        for round in s.split('\n'):
            raw = round.split(' ')
            op = ord(raw[0]) - 65
            self = ord(raw[1]) - 88
            result = (self - op + 1) % 3 - 1

            score += self + 1

            if result == 1:
                score += 6
            if result == 0:
                score += 3

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
