from tool.runners.python import SubmissionPy

class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        buf_size = 4
        for i in range(len(s) - buf_size + 1):
        	if len(set(s[i:i+buf_size])) == buf_size:
        		return i + buf_size


def test_didip():
    """
    Run `python -m pytest ./day-06/part-1/david.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """mjqjpqmgbljsphdztnvjfqwrcgsmlb""".strip()
        )
        == 7
    )