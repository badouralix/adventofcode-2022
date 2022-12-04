from tool.runners.python import SubmissionPy


def is_contained(p1_start, p1_end, p2_start, p2_end):
    return (p1_start <= p2_start and p2_end <= p1_end) or (
        p2_start <= p1_start and p1_end <= p2_end
    )


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        contained = 0
        for pair in s.splitlines():
            first, second = pair.split(",")
            p1_start, p1_end = first.split("-")
            p2_start, p2_end = second.split("-")
            if is_contained(int(p1_start), int(p1_end), int(p2_start), int(p2_end)):
                contained += 1
        return contained


def test_th_ch():
    """
    Run `python -m pytest ./day-04/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
""".strip()
        )
        == 2
    )
