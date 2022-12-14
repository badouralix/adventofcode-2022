from tool.runners.python import SubmissionPy
from collections import defaultdict


class JulesdSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        scan = defaultdict(lambda: defaultdict(bool))
        biggestY = 0
        for line in s.split("\n"):
            data = line.split(" -> ")
            for i in range(1, len(data)):
                a,b = map(int, data[i-1].split(','))
                x,y = map(int, data[i].split(','))
                if x != a:
                    for i in range(min(a, x), max(a, x) + 1):
                        scan[i][b] = True
                elif y != b:
                    for i in range(min(b, y), max(b, y)+1):
                        scan[a][i] = True
                    biggestY = max(biggestY, max(b, y))
        pouring_point = (500,0)
        current_sand = pouring_point
        total = 1
        while True:
            x, y = current_sand
            # Next level is the floor, so we set it to True
            if y == biggestY + 1:
                scan[x][y+1] = True
                scan[x-1][y+1] = True
                scan[x+1][y+1] = True
            if not scan[x][y+1]:
                current_sand = (x, y+1)
            elif not scan[x-1][y+1]:
                current_sand = (x-1, y+1)
            elif not scan[x+1][y+1]:
                current_sand = (x+1, y+1)
            else:
                if current_sand == pouring_point:
                    return total
                scan[x][y] = True
                current_sand = pouring_point
                total += 1


def test_julesd():
    """
    Run `python -m pytest ./day-14/part-1/julesd.py` to test the submission.
    """
    assert (
        JulesdSubmission().run(
            """
""".strip()
        )
        == None
    )
