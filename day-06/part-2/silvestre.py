from tool.runners.python import SubmissionPy
from collections import Counter

class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        counter = Counter(s[:14])
        i = 13
        while i < len(s):
            if len(counter) == 14:
                return i+1
            if counter[s[i-13]] == 1:
                counter.pop(s[i-13])
            else:
                counter[s[i-13]] -= 1
            counter[s[i+1]] += 1
            i += 1
        return len(s)


def test_silvestre():
    """
    Run `python -m pytest ./day-06/part-1\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """
""".strip()
        )
        == None
    )
