from tool.runners.python import SubmissionPy

NB_KNOTS = 10


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        knots_pos = [[0, 0] for _ in range(NB_KNOTS)]  # all knots
        t_positions = set()
        for instruction in s.splitlines():
            direction, nb_steps = instruction.split()
            index = 1 if direction == "U" or direction == "D" else 0
            sign = 1 if direction == "U" or direction == "R" else -1
            for _ in range(int(nb_steps)):
                # move h
                knots_pos[0][index] += sign
                # does the other knots need to move?
                for k in range(1, len(knots_pos)):
                    should_move = (
                        abs(knots_pos[k - 1][0] - knots_pos[k][0]) > 1
                        or abs(knots_pos[k - 1][1] - knots_pos[k][1]) > 1
                    )
                    if should_move:
                        knots_pos[k][0] += min(
                            1, max(-1, knots_pos[k - 1][0] - knots_pos[k][0])
                        )
                        knots_pos[k][1] += min(
                            1, max(-1, knots_pos[k - 1][1] - knots_pos[k][1])
                        )
                    if k == len(knots_pos) - 1:
                        t_positions.add(tuple(knots_pos[k]))

        return len(t_positions)


def test_th_ch():
    """
    Run `python -m pytest ./day-09/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
""".strip()
        )
        == 36
    )
