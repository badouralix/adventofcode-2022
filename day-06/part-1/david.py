from tool.runners.python import SubmissionPy

class DavidSubmission(SubmissionPy):
    STEP = 4
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        letters = list(s[:self.STEP])
        idx = self.STEP
        while len(set(letters)) != self.STEP:
            letters.append(s[idx])
            letters = letters[1:]
            idx += 1
        return idx


def test_david():
    """
    Run `python -m pytest ./day-06/part-1/david.py` to test the submission.
    """
    assert (
        DavidSubmission().run(
            """
""".strip()
        )
        == None
    )
