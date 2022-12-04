from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):

    ROCK = 0
    PAPER = 1
    SCISSORS = 2

    SYMBOLS_MAP = {
        "A": ROCK,
        "B": PAPER,
        "C": SCISSORS,

        "X": ROCK,
        "Y": PAPER,
        "Z": SCISSORS,
    }

    def process_line(self, line):
        other, me = self.SYMBOLS_MAP[line[0]], self.SYMBOLS_MAP[line[2]]
        return me + self.play(other, me) + 1
    
    def play(self, other, me):
        return ((((me - other) % 3) + 1) % 3) * 3

    def run(self, s):
        return sum(self.process_line(line) for line in s.split("\n"))

def test_david():
    """
    Run `python -m pytest ./day-02/part-1/david.py` to test the submission.
    """
    assert (
        DavidSubmission().run(
            """
""".strip()
        )
        == None
    )
