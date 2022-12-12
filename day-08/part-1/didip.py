from tool.runners.python import SubmissionPy

class DidipSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        grid = s.splitlines()
        seen = [[False for _ in range(len(grid[0]))] for _ in range(len(grid))]
        counter = 0

        for i in range(len(grid)):
            max_left = -1
            j = 0
            while max_left < 9 and j < len(grid[0]):
                if int(grid[i][j]) > max_left:
                    max_left = int(grid[i][j])
                    if not seen[i][j]:
                        seen[i][j] = True
                        counter += 1
                j += 1

            max_right = -1
            j = -1
            while max_right < 9 and -j <= len(grid[0]):
                # print(i, j, grid[i], grid[i][j])
                if int(grid[i][j]) > max_right:
                    max_right = int(grid[i][j])
                    if not seen[i][j]:
                        seen[i][j] = True
                        counter += 1
                j -= 1


        for j in range(len(grid[0])):
            max_top = -1
            i = 0
            while max_top < 9 and i < len(grid[0]):
                if int(grid[i][j]) > max_top:
                    max_top = int(grid[i][j])
                    if not seen[i][j]:
                        seen[i][j] = True
                        counter += 1
                i += 1

            max_bottom = -1
            i = -1
            while max_bottom < 9 and -i <= len(grid[0]):
                if int(grid[i][j]) > max_bottom:
                    max_bottom = int(grid[i][j])
                    if not seen[i][j]:
                        seen[i][j] = True
                        counter += 1
                i -= 1

        return counter

def test_didip():
    """
    Run `python -m pytest ./day-08/part-1/didip.py` to test the submission.
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
        == 21
    )

