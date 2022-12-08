from tool.runners.python import SubmissionPy


class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        grid = [[int(c) for c in line] for line in s.split('\n')]
        view_top_grid = [[0 for _ in range(len(grid[0]))] for _ in range(len(grid))]
        view_bottom_grid = [[0 for _ in range(len(grid[0]))] for _ in range(len(grid))]
        view_left_grid = [[0 for _ in range(len(grid[0]))] for _ in range(len(grid))]
        view_right_grid = [[0 for _ in range(len(grid[0]))] for _ in range(len(grid))]
        for i in range(len(grid)):
            for j in range(len(grid[0])):
                i1, j1 = i, j
                i2, j2 = len(grid) - i - 1, len(grid[0]) - j - 1
                top_left = grid[i1][j1]
                bottom_right = grid[i2][j2]

                if j1 > 0:
                    view_left_grid[i1][j1] = 1
                j_left = j1 - 1
                while j_left > 0:
                    left = grid[i1][j_left]
                    if left >= top_left:
                        break
                    else:
                        view_left = view_left_grid[i1][j_left]
                        view_left_grid[i1][j1] += view_left
                        j_left -= view_left

                if i1 > 0:
                    view_top_grid[i1][j1] = 1
                i_top = i1 - 1
                while i_top > 0:
                    top = grid[i_top][j1]
                    if top >= top_left:
                        break
                    else:
                        view_top = view_top_grid[i_top][j1]
                        view_top_grid[i1][j1] += view_top
                        i_top -= view_top

                if j2 < len(grid[0]) - 1:
                    view_right_grid[i2][j2] = 1
                j_right = j2 + 1
                while j_right < len(grid[0]) - 1:
                    right = grid[i2][j_right]
                    if right >= bottom_right:
                        break
                    else:
                        view_right = view_right_grid[i2][j_right]
                        view_right_grid[i2][j2] += view_right
                        j_right += view_right

                if i2 < len(grid) - 1:
                    view_bottom_grid[i2][j2] = 1
                i_bottom = i2 + 1
                while i_bottom < len(grid) - 1:
                    bottom = grid[i_bottom][j2]
                    if bottom >= bottom_right:
                        break
                    else:
                        view_bottom = view_bottom_grid[i_bottom][j2]
                        view_bottom_grid[i2][j2] += view_bottom
                        i_bottom += view_bottom

        return max(view_left_grid[i][j] * view_top_grid[i][j] * view_right_grid[i][j] * view_bottom_grid[i][j] for j in range(len(grid[0])) for i in range(len(grid)))


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
