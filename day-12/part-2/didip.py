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
    def insert_in_explore(to_explore, point):
        (node, depth, previous) = point

        for i in range(len(to_explore)):
            if to_explore[i][1] >= depth:

                return to_explore[:i] + [point] + to_explore[i:]

        return to_explore + [point]


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
            # pprint(visited)
            # print(f'checking {node} from {previous}')
            visited[node] = previous
            for neighbour in self.get_neighbours(node, grid):
                if neighbour not in visited and not self.is_in(to_explore, neighbour):
                    if self.grid_value(grid, neighbour) <= self.grid_value(grid, node) + 1:
                        if neighbour == end:
                            best_depth = min(best_depth, depth + 1)
                        elif self.grid_value(grid, neighbour) == 0:
                            to_explore = self.insert_in_explore(to_explore, (neighbour, 0, node))
                            # to_explore.append((neighbour, 0, node))

                        else:
                            to_explore = self.insert_in_explore(to_explore, (neighbour, depth + 1, node))
                        # to_explore.append((neighbour, depth + 1, node))

        # # print the path
        # solution = [['.' for j in range(len(lines[0]))] for i in range(len(lines))]
        # current_point = end
        # i = 0
        # # pprint(visited)
        # while current_point != start:
        #     # print(current_point)
        #     solution[current_point[0]][current_point[1]] = str(i % 10)
        #     current_point = visited[current_point]
        #     i += 1
        # # print('best', i)

        # solution[start[0]][start[1]] = 'S'
        # solution[end[0]][end[1]] = 'E'

        # if len(grid[0]) > 151 and grid[34][151] == ord('g') - ord('a'):
        #     for line in solution:
        #         print(''.join(line))

        # print()


        return best_depth

def test_didip():
    """
    Run `python -m pytest ./day-12/part-2/didip.py` to test the submission.
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
        == 29
    )
