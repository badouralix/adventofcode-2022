from tool.runners.python import SubmissionPy
from typing import Tuple, List

class DavidSubmission(SubmissionPy):
    def scenic_score(self, grid, i0, j0):
        n, m = len(grid), len(grid[0])
        return (
            self.partial_scenic_score(grid, i0, j0, [(i0, j) for j in range(j0-1, -1, -1)]) *
            self.partial_scenic_score(grid, i0, j0, [(i0, j) for j in range(j0+1, m)]) *
            self.partial_scenic_score(grid, i0, j0, [(i, j0) for i in range(i0-1, -1, -1)]) *
            self.partial_scenic_score(grid, i0, j0, [(i, j0) for i in range(i0+1, n)])
        )
        
    def partial_scenic_score(self, grid, i0, j0, path):
        h = grid[i0][j0]
        score = 0
        for i, j in path:
            score += 1
            if grid[i][j] >= h:
                return score
        return score
        

    def run(self, s: str):
        """
        :param s: input in string format
        :return: solution flag
        """
        grid = [[int(x) for x in line] for line in s.split("\n")]
        n, m = len(grid), len(grid[0])

        result = None
        for i0 in range(n):
            for j0 in range(m):
                score = self.scenic_score(grid, i0, j0)
                if result is None or result < score:
                    result = score
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
