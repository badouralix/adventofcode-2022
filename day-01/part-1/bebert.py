from tool.runners.python import SubmissionPy


class BebertSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        maximum = 0
        current_batch = 0
        for line in s.splitlines():
            if line == '':
                if current_batch > maximum:
                    maximum = current_batch
                current_batch = 0
            else:
                current_batch += int(line)
        if current_batch > maximum:
            maximum = current_batch
        return maximum


def test_bebert():
    """
    Run `python -m pytest ./day-01/part-1/bebert.py` to test the submission.
    """
    assert (
            BebertSubmission().run(
                """1
    2
    
    2
    1
    
    5""".strip()
            )
            == 5
    )
