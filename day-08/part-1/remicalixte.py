from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        grid = [[int(c) for c in line] for line in s.split('\n')]
        visible_grid = [[0 for _ in range(len(grid[0]))] for _ in range(len(grid))]
        max_height_top = [-1 for _ in range(len(grid[0]))]
        max_height_bottom = [-1 for _ in range(len(grid[0]))]
        max_height_left = [-1 for _ in range(len(grid))]
        max_height_right = [-1 for _ in range(len(grid))]
        for i in range(len(grid)):
            for j in range(len(grid[0])):
                i1, j1 = i, j
                i2, j2 = len(grid) - i - 1, len(grid[0]) - j - 1
                top_left = grid[i1][j1]
                bottom_right = grid[i2][j2]
                if top_left > max_height_top[j1]:
                    max_height_top[j1] = top_left
                    visible_grid[i1][j1] = 1
                if top_left > max_height_left[i1]:
                    max_height_left[i1] = top_left
                    visible_grid[i1][j1] = 1
                if bottom_right > max_height_bottom[j2]:
                    max_height_bottom[j2] = bottom_right
                    visible_grid[i2][j2] = 1
                if bottom_right > max_height_right[i2]:
                    max_height_right[i2] = bottom_right
                    visible_grid[i2][j2] = 1

        return sum(sum(v for v in line) for line in visible_grid)




def test_remicalixte():
    """
    Run `python -m pytest ./day-08/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
