from tool.runners.python import SubmissionPy


class JonSubmission(SubmissionPy):
    def run(self, s):
        instructions = s.strip().split("\n")

        to_collect = {20, 60, 100, 140, 180, 220}
        result = 0

        x = 1
        cycle = 1
        pc = 0
        wait_done = False

        while pc < len(instructions):
            if cycle in to_collect:
                result += cycle * x
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

        return result


def test_jon():
    """
    Run `python -m pytest ./day-10/part-1/jon.py` to test the submission.
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
        == 13140
    )
