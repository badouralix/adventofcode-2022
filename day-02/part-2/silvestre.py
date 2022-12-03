from tool.runners.python import SubmissionPy

round_scores = {
    "A X": 0+3,
    "A Y": 3+1,
    "A Z": 6+2,
    "B X": 0+1,
    "B Y": 3+2,
    "B Z": 6+3,
    "C X": 0+2,
    "C Y": 3+3,
    "C Z": 6+1,
}

class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        return sum(round_scores[line] for line in s.split('\n'))


def test_silvestre():
    """
    Run `python -m pytest ./day-02/part-2\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """
""".strip()
        )
        == None
    )
