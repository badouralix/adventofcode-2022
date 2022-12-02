from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def get_game_score(self, opponent, us):
        if (
            (opponent == "A" and us == "X")
            or (opponent == "B" and us == "Y")
            or (opponent == "C" and us == "Z")
        ):
            return 3

        if (
            (opponent == "A" and us == "Y")
            or (opponent == "B" and us == "Z")
            or (opponent == "C" and us == "X")
        ):
            return 6

        return 0

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        total_score = 0
        for game in s.splitlines():
            opponent, us = game.split()
            score = self.get_game_score(opponent, us)
            if us == "X":
                score += 1
            elif us == "Y":
                score += 2
            else:
                score += 3
            total_score += score
        return total_score


def test_th_ch():
    """
    Run `python -m pytest ./day-02/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
A Y
B X
C Z
""".strip()
        )
        == 15
    )
