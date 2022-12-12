from tool.runners.python import SubmissionPy
from collections import deque

class RemicalixteSubmission(SubmissionPy):
    def run(self, s):
        grid = []
        starts = []
        end = (0,0)
        for i, line in enumerate(s.split('\n')):
            grid_line = []
            for j, c in enumerate(line):
                elevation = 0
                if c == 'S':
                    start = (i,j)
                    elevation = 0
                elif c == 'E':
                    end = (i,j)
                    elevation = 25
                else:
                    elevation = ord(c)-ord('a')

                if elevation == 0:
                    starts.append((i,j))
                grid_line.append(elevation)
            grid.append(grid_line)


        to_visit = deque()
        to_visit.append((end, 0))
        visited = set()
        visited.add(end)

        dist = []

        while len(to_visit) > 0:
            ((i,j), d) = to_visit.popleft()
            neighbors = get_neighbors(grid, (i,j))
            for neighbor in neighbors:
                if neighbor in visited:
                    continue
                if neighbor in starts:
                    dist.append(d+1)
                to_visit.append((neighbor, d+1))
                visited.add(neighbor)



        return min(dist)


def get_neighbors(grid, coor):
    neighbors = []
    i,j = coor
    elevation = grid[i][j]
    potentials = []
    if i > 0:
        potentials.append((i-1,j))
    if j > 0:
        potentials.append((i,j-1))
    if i < len(grid) -1:
        potentials.append((i+1,j))
    if j < len(grid[0]) -1:
        potentials.append((i,j+1))

    for potential in potentials:
        pi,pj = potential
        if elevation - grid[pi][pj] <= 1:
            neighbors.append((pi,pj))

    return neighbors








def test_remicalixte():
    """
    Run `python -m pytest ./day-12/part-1/remicalixte.py` to test the submission.
    """
    assert (
        RemicalixteSubmission().run(
            """
""".strip()
        )
        == None
    )
