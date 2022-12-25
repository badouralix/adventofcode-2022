from tool.runners.python import SubmissionPy

SNAFU_CHARS = {
    "2": 2,
    "1": 1,
    "0": 0,
    "-": -1,
    "=": -2,
}


def decimal_to_snafu(decimal):
    # Convert it to base 5
    digits = []
    while decimal:
        digit = decimal % 5
        digits.append(digit)
        decimal //= 5

    # Go from [0, 4] to [-2, 2] for each digit
    snafu_chars = []
    buffer = 0
    for digit in digits:
        digit += buffer
        buffer = 0
        if digit == 3:
            snafu_chars.append("=")
            buffer += 1
        elif digit == 4:
            snafu_chars.append("-")
            buffer += 1
        elif digit == 5:
            snafu_chars.append("0")
            buffer += 1
        else:
            snafu_chars.append(str(digit))
    if buffer > 0:
        snafu_chars.append(str(buffer))

    return "".join(snafu_chars[::-1])


def snafu_to_decimal(snafu):
    decimal = 0
    factor = 1
    for char in reversed(snafu):
        decimal += SNAFU_CHARS[char] * factor
        factor *= 5
    return decimal


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        decimal = sum(snafu_to_decimal(snafu) for snafu in s.splitlines())
        return decimal_to_snafu(decimal)


def test_th_ch():
    """
    Run `python -m pytest ./day-25/part-1/th-ch.py` to test the submission.
    """
    assert snafu_to_decimal("1121-1110-1=0") == 314159265

    assert decimal_to_snafu(1) == "1"
    assert decimal_to_snafu(2) == "2"
    assert decimal_to_snafu(3) == "1="
    assert decimal_to_snafu(4) == "1-"
    assert decimal_to_snafu(5) == "10"
    assert decimal_to_snafu(6) == "11"
    assert decimal_to_snafu(7) == "12"
    assert decimal_to_snafu(8) == "2="
    assert decimal_to_snafu(9) == "2-"
    assert decimal_to_snafu(10) == "20"
    assert decimal_to_snafu(15) == "1=0"
    assert decimal_to_snafu(20) == "1-0"
    assert decimal_to_snafu(2022) == "1=11-2"
    assert decimal_to_snafu(12345) == "1-0---0"
    assert decimal_to_snafu(314159265) == "1121-1110-1=0"

    assert (
        ThChSubmission().run(
            """
1=-0-2
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
