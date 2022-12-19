from tool.runners.python import SubmissionPy
from pprint import pprint
import math

class DidipSubmission(SubmissionPy):
    @staticmethod
    def is_in(grid, node_to_find):
        for (node, _, _) in grid:
            if node == node_to_find:
                return True

        return False

    @staticmethod
    def grid_value(grid, node):
        return grid[node[0]][node[1]]


    @staticmethod
    def get_neighbours(node, grid):
        neighbours = []
        if node[0] > 0:
            neighbours.append((node[0] - 1, node[1]))
        if node[0] < len(grid) - 1:
            neighbours.append((node[0] + 1, node[1]))
        if node[1] > 0:
            neighbours.append((node[0], node[1] - 1))
        if node[1] < len(grid[0]) - 1:
            neighbours.append((node[0], node[1] + 1))

        return neighbours

    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Your code goes here
        lines = s.splitlines()
        grid = [[None for _ in range(len(lines[0]))] for _ in range(len(lines))]

        for x in range(len(lines)):
            for y in range(len(lines[0])):
                if lines[x][y] not in ['S', 'E']:
                    grid[x][y] = ord(lines[x][y]) - ord('a')
                elif lines[x][y] == 'S':
                    start = (x, y)
                    grid[x][y] = 0
                else:
                    end = (x, y)
                    grid[x][y] = 25

        to_explore = [(start, 0, start)]
        visited = {}
        best_depth = math.inf
        # pprint(grid)

        while len(to_explore) > 0:
            node, depth, previous = to_explore.pop(0)
            visited[node] = previous
            for neighbour in self.get_neighbours(node, grid):
                if neighbour not in visited and not self.is_in(to_explore, neighbour):
                    if self.grid_value(grid, neighbour) <= self.grid_value(grid, node) + 1:
                        if neighbour == end:
                            return depth + 1
                            # best_depth = min(best_depth, depth + 1)

                        to_explore.append((neighbour, depth + 1, node))


def test_didip():
    """
    Run `python -m pytest ./day-12/part-1/didip.py` to test the submission.
    """
    assert (
        DidipSubmission().run(
"""
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
""".strip()
        )
        == 31
    )
