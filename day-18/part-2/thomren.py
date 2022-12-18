from collections import defaultdict, deque
from tool.runners.python import SubmissionPy


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        droplets = {tuple(map(int, line.split(","))) for line in s.splitlines()}
        faces = [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]

        xmin = min(x for (x, _, _) in droplets)
        xmax = max(x for (x, _, _) in droplets)
        ymin = min(y for (_, y, _) in droplets)
        ymax = max(y for (_, y, _) in droplets)
        zmin = min(z for (_, _, z) in droplets)
        zmax = max(z for (_, _, z) in droplets)

        start = (
            xmin - 1,
            ymin - 1,
            zmin - 1,
        )

        # bfs of the area around the droplet to count the number of exterior faces
        # we explore [xmin-1;xmax+1]x[ymin-1;ymin+1]x[zmin-1;zmax+1]
        queue = deque([start])
        seen = set()
        exterior_surface = 0
        while len(queue) > 0:
            (x, y, z) = queue.popleft()
            for (dx, dy, dz) in faces:
                neighbor = (x + dx, y + dy, z + dz)
                if (
                    neighbor not in droplets
                    and neighbor not in seen
                    and xmin - 1 <= neighbor[0] <= xmax + 1
                    and ymin - 1 <= neighbor[1] <= ymax + 1
                    and zmin - 1 <= neighbor[2] <= zmax + 1
                ):
                    queue.append(neighbor)
                    seen.add(neighbor)
                elif neighbor in droplets:
                    exterior_surface += 1

        return exterior_surface


def test_thomren():
    """
    Run `python -m pytest ./day-18/part-2/thomren.py` to test the submission.
    """
    assert ThomrenSubmission().run("1,1,1\n2,1,1") == 10

    assert (
        ThomrenSubmission().run(
            """2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
""".strip()
        )
        == 58
    )
