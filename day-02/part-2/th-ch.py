from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def find_move_to_play(self, opponent, output):
        # rock
        if opponent == "A":
            # lose
            if output == "X":
                return 3
            # draw
            elif output == "Y":
                return 1
            else:
                return 2
        # paper
        elif opponent == "B":
            # lose
            if output == "X":
                return 1
            # draw
            elif output == "Y":
                return 2
            else:
                return 3
        # scissors
        else:
            # lose
            if output == "X":
                return 2
            # draw
            elif output == "Y":
                return 3
            else:
                return 1

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        total_score = 0
        for game in s.splitlines():
            opponent, output = game.split()
            move_score = self.find_move_to_play(opponent, output)
            if output == "X":
                output_score = 0
            elif output == "Y":
                output_score = 3
            else:
                output_score = 6
            total_score += move_score + output_score
        return total_score


def test_th_ch():
    """
    Run `python -m pytest ./day-02/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
A Y
B X
C Z
""".strip()
        )
        == 12
    )
