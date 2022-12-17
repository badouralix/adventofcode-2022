from tool.runners.python import SubmissionPy
import numpy as np
from typing import Union, Tuple, Set, List

Node = Tuple[int, int]


def isvalid(grid: np.array, pos: Tuple[int, int], neighbour: Tuple[int, int]) -> bool:
    return grid[pos[0]][pos[1]] - grid[neighbour[0]][neighbour[1]] <= 1


def neighbors(grid: np.array, node: Node) -> Set[Node]:
    adjacent_positions = set()
    if node[0] > 0:
        adjacent_positions.add((node[0] - 1, node[1]))
    if node[0] < grid.shape[0] - 1:
        adjacent_positions.add((node[0] + 1, node[1]))
    if node[1] > 0:
        adjacent_positions.add((node[0], node[1] - 1))
    if node[1] < grid.shape[1] - 1:
        adjacent_positions.add((node[0], node[1] + 1))
    return {e for e in adjacent_positions if isvalid(grid, node, e)}


def BFS(grid: np.array, end: Node, start: Node) -> int:
    queue: List[Tuple[Node, int]] = []
    visited: Set[Node] = set()
    queue.append((end, 0))
    visited.add(end)
    while len(queue) > 0:
        current_node, steps = queue.pop(0)
        for adjacent in neighbors(grid, current_node):
            if current_node == start:
                return steps
            if adjacent not in visited:
                visited.add(adjacent)
                queue.append((adjacent, steps + 1))
    return 0


class YouyounSubmission(SubmissionPy):
    def run(self, s):
        grid = np.array([[ord(e) - ord('a') for e in list(x)] for x in s.split("\n")])
        start = tuple(e.item() for e in np.where(grid == ord('S') - ord('a')))
        end = tuple(e.item() for e in np.where(grid == ord('E') - ord('a')))
        grid[start] = 0
        grid[end] = 25
        steps = BFS(grid, end, start)
        return steps


def test_youyoun():
    """
    Run `python -m pytest ./day-12/part-2/youyoun.py` to test the submission.
    """
    assert (
            YouyounSubmission().run(
                """
    """.strip()
            )
            == None
    )
