from tool.runners.python import SubmissionPy
from pprint import pprint

class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        grid = s.splitlines()
        width, height = len(grid[0]), len(grid)

        view_top = [[0 for _ in range(width)] for _ in range(height)]
        view_bottom = [[0 for _ in range(width)] for _ in range(height)]
        view_left = [[0 for _ in range(width)] for _ in range(height)]
        view_right = [[0 for _ in range(width)] for _ in range(height)]


        for i in range(1, height):
            for j in range(width):
                current_view_top = 1
                while int(grid[i - current_view_top][j]) < int(grid[i][j]) and i - current_view_top > 0:
                    current_view_top += view_top[i - current_view_top][j]

                view_top[i][j] = current_view_top


        for i in range(-2, -height - 1, -1):
            for j in range(width):
                current_view_bottom = 1
                while int(grid[i + current_view_bottom][j]) < int(grid[i][j]) and -i - current_view_bottom > 1:
                    current_view_bottom += view_bottom[i + current_view_bottom][j]

                view_bottom[i][j] = current_view_bottom

        for j in range(1, width):
            for i in range(height):
                current_view_left = 1
                while int(grid[i][j - current_view_left]) < int(grid[i][j]) and j - current_view_left > 0:
                    current_view_left += view_left[i][j - current_view_left]

                view_left[i][j] = current_view_left


        for j in range(-2, -width - 1, -1):
            for i in range(height):
                current_view_right = 1
                while int(grid[i][j + current_view_right]) < int(grid[i][j]) and -j - current_view_right > 1:
                    current_view_right += view_right[i][j + current_view_right]

                view_right[i][j] = current_view_right

        current_max = 0
        for i in range(1, height - 1):
            for j in range(1, width - 1):
                view = view_top[i][j] * view_bottom[i][j] * view_left[i][j] * view_right[i][j]
                if view > current_max:
                    current_max = view

        return current_max


def test_didip():
    """
    Run `python -m pytest ./day-08/part-2/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
            """30373
25512
65332
33549
35390
""".strip()
        )
        == 8
    )
