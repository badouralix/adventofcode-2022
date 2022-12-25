from math import log
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        return dec_to_snafu(sum(snafu_to_dec(x) for x in s.splitlines()))


def dec_to_snafu(n: int) -> str:
    res = ""
    r = 0
    while n > 0:
        n, d = divmod(n, 5)
        d += r
        r = int(d >= 3)
        if d == 5:
            r = 1
            res += "0"
        elif d == 4:
            res += "-"
        elif d == 3:
            res += "="
        else:
            res += str(d)
    res += "1" if r else ""
    return res[::-1]


def test_dec_to_snafu():
    assert [dec_to_snafu(n) for n in range(1, 11)] == [
        "1",
        "2",
        "1=",
        "1-",
        "10",
        "11",
        "12",
        "2=",
        "2-",
        "20",
    ]
    assert dec_to_snafu(15) == "1=0"
    assert dec_to_snafu(20) == "1-0"
    assert dec_to_snafu(2022) == "1=11-2"
    assert dec_to_snafu(12345) == "1-0---0"
    assert dec_to_snafu(314159265) == "1121-1110-1=0"


def snafu_to_dec(s: str) -> int:
    res = 0
    p = 1
    for d in s[::-1]:
        if d.isdigit():
            res += p * int(d)
        elif d == "-":
            res -= p
        elif d == "=":
            res -= 2 * p
        else:
            raise ValueError(f"invalid digit: {d}")
        p *= 5
    return res


def test_snafu_to_dec():
    assert [
        snafu_to_dec(n)
        for n in [
            "1",
            "2",
            "1=",
            "1-",
            "10",
            "11",
            "12",
            "2=",
            "2-",
            "20",
        ]
    ] == list(range(1, 11))
    assert snafu_to_dec("1=0") == 15
    assert snafu_to_dec("1-0") == 20
    assert snafu_to_dec("1=11-2") == 2022
    assert snafu_to_dec("1-0---0") == 12345
    assert snafu_to_dec("1121-1110-1=0") == 314159265


def test_thomren():
    """
    Run `python -m pytest ./day-25/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
""".strip()
        )
        == "2=-1=0"
    )
