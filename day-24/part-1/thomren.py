from collections import defaultdict, deque
import functools
import heapq
from typing import Deque, Dict, List, Tuple
from tool.runners.python import SubmissionPy

DIRECTIONS = {">": (0, 1), "<": (0, -1), "^": (-1, 0), "v": (1, 0)}

Coord2D = Tuple[int, int]
BlizzardsMap = Dict[Coord2D, List[Coord2D]]


class ThomrenSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        lines = s.splitlines()
        height, width = len(lines) - 2, len(lines[0]) - 2
        start = (-1, next(y for y in range(width + 1) if lines[0][y] == ".") - 1)
        end = (height, next(y for y in range(width + 1) if lines[-1][y] == ".") - 1)
        blizzards = {
            (x - 1, y - 1): [DIRECTIONS[lines[x][y]]]
            for x in range(1, height + 1)
            for y in range(1, width + 1)
            if lines[x][y] in ">^v<"
        }

        # memoize the blizzard states at each minute
        @functools.cache
        def get_blizzard(minute: int) -> BlizzardsMap:
            if minute == -1:
                return blizzards
            return blizzards_move(get_blizzard(minute - 1), height, width)

        # A* algorithm with manhattan distance heuristic to find the shortest path
        queue: List[Tuple[int, int, Coord2D]] = [(manhattan_dist(start, end), 0, start)]
        seen = set()
        while len(queue) > 0:
            _, dist, pos = heapq.heappop(queue)

            if pos == end:
                return dist

            if (dist, pos) in seen:
                continue
            seen.add((dist, pos))

            blizzards = get_blizzard(dist)
            for (dx, dy) in [(0, 0), (0, -1), (-1, 0), (1, 0), (0, 1)]:
                x = pos[0] + dx
                y = pos[1] + dy
                if (x, y) not in blizzards and (
                    0 <= x < height and 0 <= y < width or (x, y) in (start, end)
                ):
                    heapq.heappush(
                        queue,
                        (
                            dist + 1 + manhattan_dist(pos, end),
                            dist + 1,
                            (x, y),
                        ),
                    )

        return 1


def blizzards_move(blizzards: BlizzardsMap, height: int, width: int) -> BlizzardsMap:
    next_blizzards = defaultdict(list)
    for (x, y), directions in list(blizzards.items()):
        for (dx, dy) in directions:
            next_x = (x + dx) % height
            next_y = (y + dy) % width
            next_blizzards[(next_x, next_y)].append((dx, dy))
    return next_blizzards


def manhattan_dist(p: Coord2D, q: Coord2D) -> int:
    return sum(abs(a - b) for a, b in zip(p, q))


def test_thomren():
    """
    Run `python -m pytest ./day-24/part-1/thomren.py` to test the submission.
    """
    assert (
        ThomrenSubmission().run(
            """#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
""".strip()
        )
        == 18
    )
