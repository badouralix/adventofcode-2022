from tool.runners.python import SubmissionPy

symbol_scores = {
    "X": 1,
    "Y": 2,
    "Z": 3,
}

win_scores = {
    "A X": 3,
    "B Y": 3,
    "C Z": 3,
    "A Y": 6,
    "B Z": 6,
    "C X": 6,
    "A Z": 0,
    "B X": 0,
    "C Y": 0,
}


class BebertSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        score = 0
        for line in s.splitlines():
            win_score = win_scores[line]
            symbol_score = symbol_scores[line[2]]
            score += win_score + symbol_score
        return score


def test_bebert():
    """
    Run `python -m pytest ./day-02/part-1/bebert.py` to test the submission.
    """
    assert (
        BebertSubmission().run(
            """
A Y
B X
C Z
""".strip()
        )
        == 15
    )
