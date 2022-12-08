from tool.runners.python import SubmissionPy
import numpy as np

class SilvestreSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        height_grid = np.vstack([np.fromiter(line, dtype="int8") for line in s.split("\n")])
        
        scores = np.zeros([*height_grid.shape, 4], dtype="int32") # ask once 
        up_scores = scores[:, :, 0]
        down_scores = scores[:, :, 1]
        left_scores = scores[:, :, 2]
        right_scores = scores[:, :, 3]
        
        # top -> bottom
        for i in range(height_grid.shape[0]-1):
            down_scores[i, :] = 1 + np.logical_and.accumulate(
                    height_grid[i, :] > height_grid[i+1:, :],
                    axis=0).sum(axis=0)
            down_scores[i, :] = np.minimum(down_scores[i, :], height_grid.shape[0] - (i+1))
            
        # bottom -> top
        for i in range(height_grid.shape[0]-1, 0, -1):
            up_scores[i, :] = 1 + np.logical_and.accumulate(
                    height_grid[i, :] > height_grid[i-1::-1, :],
                    axis=0).sum(axis=0)
            up_scores[i, :] = np.minimum(up_scores[i, :], i)

        # left -> right
        for j in range(height_grid.shape[1]-1):
            right_scores[:, j] = 1 + np.logical_and.accumulate(
                    height_grid[:, j] > height_grid[:, j+1:].T,
                    axis=0).sum(axis=0)
            right_scores[:, j] = np.minimum(right_scores[:, j], height_grid.shape[1] - (j+1))
            
        # right -> left
        for j in range(height_grid.shape[1]-1, 0, -1):
            left_scores[:, j] = 1 + np.logical_and.accumulate(
                    height_grid[:, j] > height_grid[:, j-1::-1].T,
                    axis=0).sum(axis=0)
            left_scores[:, j] = np.minimum(left_scores[:, j], j)
        
        return np.max(up_scores * down_scores * left_scores * right_scores)


def test_silvestre():
    """
    Run `python -m pytest ./day-08/part-2\silvestre.py` to test the submission.
    """
    assert (
        SilvestreSubmission().run(
            """
""".strip()
        )
        == None
    )
