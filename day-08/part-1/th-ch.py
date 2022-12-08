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
        nb_visible_trees = 2 * h + 2 * w - 4  # edges
        for y in range(1, h - 1):
            for x in range(1, w - 1):
                tree = trees[y][x]
                is_visible = (
                    all(trees[j][x] < tree for j in range(0, y))
                    or all(trees[j][x] < tree for j in range(y + 1, h))
                    or all(trees[y][i] < tree for i in range(0, x))
                    or all(trees[y][i] < tree for i in range(x + 1, w))
                )
                if is_visible:
                    nb_visible_trees += 1

        return nb_visible_trees


def test_th_ch():
    """
    Run `python -m pytest ./day-08/part-1/th-ch.py` to test the submission.
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
        == 21
    )
