from tool.runners.python import SubmissionPy


class Clock:
    def __init__(self):
        self.cycle = 1
        self.X = 1
        self.drawn_pixels = set()

    def increase_cycle(self):
        pixel = self.cycle - 1
        if self.X - 1 <= pixel % 40 <= self.X + 1:
            # Draw
            self.drawn_pixels.add(pixel)

        self.cycle += 1

    def run_instructions(self, instructions):
        for instruction in instructions:
            if instruction == "noop":
                self.increase_cycle()
            else:
                V = int(instruction.replace("addx ", ""))
                self.increase_cycle()
                self.increase_cycle()
                self.X += V


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        clock = Clock()
        clock.run_instructions(s.splitlines())
        return "\n".join(
            "".join("#" if j * 40 + i in clock.drawn_pixels else "." for i in range(40))
            for j in range(6)
        )


def test_th_ch():
    """
    Run `python -m pytest ./day-10/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
""".strip()
        )
        == """
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
""".strip()
    )
