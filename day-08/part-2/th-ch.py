from tool.runners.python import SubmissionPy


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        trees = s.splitlines()
        w = len(trees[0])
        h = len(trees)
        max_scenic_score = 0
        for y in range(h):
            for x in range(w):
                tree = trees[y][x]

                top = next((j for j in range(1, y) if trees[y - j][x] >= tree), None)
                top = y if top is None else top

                bottom = next(
                    (j for j in range(1, h - y) if trees[y + j][x] >= tree), None
                )
                bottom = h - y - 1 if bottom is None else bottom

                left = next((i for i in range(1, x) if trees[y][x - i] >= tree), None)
                left = x if left is None else left

                right = next(
                    (i for i in range(1, w - x) if trees[y][x + i] >= tree), None
                )
                right = w - x - 1 if right is None else right

                scenic_score = top * bottom * left * right
                max_scenic_score = max(max_scenic_score, scenic_score)

        return max_scenic_score


def test_th_ch():
    """
    Run `python -m pytest ./day-08/part-2/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
30373
25512
65332
33549
35390
""".strip()
        )
        == 8
    )
