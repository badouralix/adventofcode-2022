from tool.runners.python import SubmissionPy

debug = False

class JonSubmission(SubmissionPy):
    def run(self, s):
        instructions = s.strip().split("\n")
        screen = ["." for _ in range(40*6)]

        x = 1
        cycle = 1
        pc = 0
        wait_done = False

        while pc < len(instructions):

            # Draw pixel
            crt_x = ((cycle - 1) % 40)
            if debug:
                print("Cycle={} REG={} CRT={} Print={}".format(cycle, x, crt_x, abs(x - crt_x) <= 1))
            if abs(x - crt_x) <= 1:
                screen[cycle-1] = "#"

            # Execute instruction
            args = instructions[pc].split(" ")
            if args[0] == "noop":
                pc += 1
            elif args[0] == "addx":
                if wait_done:
                    x += int(args[1])
                    pc += 1
                    wait_done = False
                else:
                    wait_done = True
            else:
                raise Exception("Unsupported op: " + args[0])

            cycle += 1

        if debug:
            for i in range(6):
                print("".join(screen[40*i:40*(i+1)]))

        return "".join(screen)


def test_jon():
    """
    Run `python -m pytest ./day-10/part-2/jon.py` to test the submission.
    """
    assert (
        JonSubmission().run(
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
""".replace("\n", "")
    )
