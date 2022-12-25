from collections import defaultdict
from queue import PriorityQueue
from tool.runners.python import SubmissionPy
from functools import cache


def apply_blizzards(w, h, all_blizzards):
    new_blizzards = defaultdict(list)
    for (x, y), blizzards in all_blizzards.items():
        for blizzard in blizzards:
            if blizzard == "^":
                new_blizzards[(x, (y - 1 + h) % h)].append(blizzard)
            elif blizzard == "v":
                new_blizzards[(x, (y + 1) % h)].append(blizzard)
            elif blizzard == ">":
                new_blizzards[((x + 1) % w, y)].append(blizzard)
            elif blizzard == "<":
                new_blizzards[((x - 1 + w) % w, y)].append(blizzard)
    return new_blizzards


@cache
def get_blizzards_by_minute(w, h, minute):
    if minute == 0:
        return get_blizzards_by_minute.starting_blizzards
    return apply_blizzards(w, h, get_blizzards_by_minute(w, h, minute - 1))


def pretty_print_blizzards(w, h, blizzards):
    repr_blizzard = (
        lambda x, y: "." if (x, y) not in blizzards else str(len(blizzards[(x, y)]))
    )
    print("\n".join("".join(repr_blizzard(x, y) for x in range(w)) for y in range(h)))


class ThChSubmission(SubmissionPy):
    def run(self, s):
        """
        :param s: input in string format
        :return: solution flag
        """
        # Clear cache to be able to run on different inputs
        get_blizzards_by_minute.cache_clear()

        starting_blizzards = defaultdict(list)
        lines = s.splitlines()
        height = len(lines) - 2
        width = len(lines[0]) - 2
        for y, line in enumerate(lines[1:-1]):
            for x, char in enumerate(line[1:-1]):
                if char != ".":
                    starting_blizzards[(x, y)].append(char)
        get_blizzards_by_minute.starting_blizzards = starting_blizzards

        # A* algorithm
        open_set = PriorityQueue()
        start = (0, -1)
        goal = (width - 1, height)
        h = lambda x, y: abs(goal[0] - x) + abs(goal[1] - y)  # Heuristic (distance)
        open_set.put((0, h(*start), start))
        seen = set()
        while not open_set.empty():
            minute, _, current = open_set.get()
            if current == goal:
                return minute

            if (current, minute) in seen:
                continue
            seen.add((current, minute))

            new_minute = minute + 1
            new_blizzards = get_blizzards_by_minute(width, height, new_minute)
            neighbors = set()
            x, y = current
            for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]:
                if (x + dx, y + dy) in new_blizzards:
                    continue
                if (
                    (0 <= x + dx < width and 0 <= y + dy < height)
                    or (x + dx, y + dy) == goal
                    or (x + dx, y + dy) == start
                ):
                    neighbors.add((x + dx, y + dy))
            for neighbor in neighbors:
                open_set.put((new_minute, h(*neighbor), neighbor))

        raise Exception("failed")


def test_th_ch():
    """
    Run `python -m pytest ./day-24/part-1/th-ch.py` to test the submission.
    """
    assert (
        ThChSubmission().run(
            """
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
""".strip()
        )
        == 18
    )
