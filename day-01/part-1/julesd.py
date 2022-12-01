from tool.runners.python import SubmissionPy


class JulesdSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        elves = []
        cur_elf = []
        for line in s.split('\n'):
            if line == "":
                elves.append(cur_elf)
                cur_elf = []
                continue
            cur_elf.append(int(line))
        return(max([sum(x) for x in elves]))


def test_julesd():
    """
    Run `python -m pytest ./day-01/part-1/julesd.py` to test the submission.
    """
    assert (
        JulesdSubmission().run(
            """
""".strip()
        )
        == None
    )
