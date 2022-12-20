from tool.runners.python import SubmissionPy

DECRYPTION_KEY = 811589153
N_MIXES = 10


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        seq = [n * DECRYPTION_KEY for n in map(int, s.splitlines())]
        indices = list(range(len(seq)))

        for _ in range(N_MIXES):
            for i in range(len(seq)):
                pos = indices.index(i)
                indices.pop(pos)
                indices.insert((pos + seq[i]) % (len(seq) - 1), i)

        pos = indices.index(seq.index(0))
        return sum(seq[indices[(pos + k * 1000) % len(seq)]] for k in [1, 2, 3])


def test_thomren():
    """
    Run `python -m pytest ./day-20/part-2/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """1
2
-3
3
-2
0
4
""".strip()
        )
        == 1623178306
    )
