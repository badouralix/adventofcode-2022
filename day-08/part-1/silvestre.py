from tool.runners.python import SubmissionPy
import numpy as np

class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        height_grid = np.vstack([np.fromiter(line, dtype="int8") for line in s.split("\n")])
        visible_grid = np.zeros_like(height_grid, dtype=bool)
        height_grid = np.pad(height_grid, ((1, 1), (1, 1)), "constant", constant_values=-1)
        
        # right -> left
        visible_grid |= height_grid[1:-1, 1:-1] > np.maximum.accumulate(height_grid[1:-1, -1:1:-1], axis=1)[:, ::-1] 
        # left -> right
        visible_grid |= height_grid[1:-1, 1:-1] > np.maximum.accumulate(height_grid[1:-1, :-2], axis=1) 
        # top -> bottom
        visible_grid |= height_grid[1:-1, 1:-1] > np.maximum.accumulate(height_grid[:-2, 1:-1], axis=0) 
        # bottom -> tom
        visible_grid |= height_grid[1:-1, 1:-1] > np.maximum.accumulate(height_grid[-1:1:-1, 1:-1], axis=0)[::-1, :] 
        return np.sum(visible_grid)

def test_silvestre():
    """
    Run `python -m pytest ./day-08/part-1\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """
""".strip()
        )
        == None
    )
