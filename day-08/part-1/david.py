from tool.runners.python import SubmissionPy


class DavidSubmission(SubmissionPy):
    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[int(x) for x in line] for line in s.split("\n")]
        n, m = len(grid), len(grid[0])

        result = 0
        for i0 in range(n):
            for j0 in range(m):
                h = grid[i0][j0]
                if (
                    all(grid[i0][j] < h for j in range(j0)) or
                    all(grid[i0][j] < h for j in range(j0+1, m)) or
                    all(grid[i][j0] < h for i in range(i0)) or
                    all(grid[i][j0] < h for i in range(i0+1, n))
                ):
                    result += 1

        return result



def test_david():
    """
    Run `python -m pytest ./day-08/part-1/david.py` to test the submission.
    """
    assert (
        DavidSubmission().run(
            """
""".strip()
        )
        == None
    )
