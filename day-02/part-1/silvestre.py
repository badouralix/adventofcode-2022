from tool.runners.python import SubmissionPy

round_scores = {
    "A X": 3+1,
    "A Y": 6+2,
    "A Z": 0+3,
    "B X": 0+1,
    "B Y": 3+2,
    "B Z": 6+3,
    "C X": 6+1,
    "C Y": 0+2,
    "C Z": 3+3,
}

class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        return sum(round_scores[line] for line in s.split('\n'))


def test_silvestre():
    """
    Run `python -m pytest ./day-02/part-1\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """
""".strip()
        )
        == None
    )
