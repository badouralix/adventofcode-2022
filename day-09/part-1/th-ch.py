from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        h_pos = [0, 0]
        t_pos = [0, 0]
        t_positions = set()
        for instruction in s.splitlines():
            direction, nb_steps = instruction.split()
            index = 1 if direction == "U" or direction == "D" else 0
            sign = 1 if direction == "U" or direction == "R" else -1
            for _ in range(int(nb_steps)):
                # move h
                h_pos[index] += sign
                # does the tail need to move?
                should_move = (
                    abs(h_pos[0] - t_pos[0]) > 1 or abs(h_pos[1] - t_pos[1]) > 1
                )
                if should_move:
                    t_pos[0] += min(1, max(-1, h_pos[0] - t_pos[0]))
                    t_pos[1] += min(1, max(-1, h_pos[1] - t_pos[1]))
                t_positions.add(tuple(t_pos))

        return len(t_positions)


def test_th_ch():
    """
    Run `python -m pytest ./day-09/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
""".strip()
        )
        == 13
    )
