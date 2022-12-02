from tool.runners.python import SubmissionPy

win_scores = {
    "A X": 0,
    "B Y": 3,
    "C Z": 6,
    "A Y": 3,
    "B Z": 6,
    "C X": 0,
    "A Z": 6,
    "B X": 0,
    "C Y": 3,
}

symbol_scores = {
    "A X": 3,
    "B Y": 2,
    "C Z": 1,
    "A Y": 1,
    "B Z": 3,
    "C X": 2,
    "A Z": 2,
    "B X": 1,
    "C Y": 3,
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
            symbol_score = symbol_scores[line]
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
            == 12
    )
