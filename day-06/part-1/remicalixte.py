from tool.runners.python import SubmissionPy
from collections import defaultdict

class RemicalixteSubmission(SubmissionPy):
    def run(self, s):

        marker = defaultdict(int)
        head = 0
        tail = 0
        while len(marker) < 4 and head < len(s):
            marker[s[head]] += 1
            head += 1
            if head - tail > 4:
                marker[s[tail]] -= 1
                if marker[s[tail]] == 0:
                    marker.pop(s[tail])
                tail += 1

        return head




def test_remicalixte():
    """
    Run `python -m pytest ./day-06/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
