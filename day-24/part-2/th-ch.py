from tool.runners.python import SubmissionPy

from collections import defaultdict
from queue import PriorityQueue

from importlib import import_module

part1 = import_module("day-24.part-1.th-ch")
apply_blizzards = part1.apply_blizzards
get_blizzards_by_minute = part1.get_blizzards_by_minute
pretty_print_blizzards = part1.pretty_print_blizzards


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

        start = (0, -1)
        end = (width - 1, height)
        minutes_after_first_trip = self.find_minutes(
            start=start,
            goal=end,
            width=width,
            height=height,
            first_minute=0,
        )
        minutes_after_second_trip = self.find_minutes(
            start=end,
            goal=start,
            width=width,
            height=height,
            first_minute=minutes_after_first_trip,
        )
        minutes_after_third_trip = self.find_minutes(
            start=start,
            goal=end,
            width=width,
            height=height,
            first_minute=minutes_after_second_trip,
        )

        return minutes_after_third_trip

    def find_minutes(self, start, goal, width, height, first_minute=0):
        # A* algorithm
        open_set = PriorityQueue()
        h = lambda x, y: abs(goal[0] - x) + abs(goal[1] - y)  # Heuristic (distance)
        open_set.put((first_minute, h(*start), start))
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
    Run `python -m pytest ./day-24/part-2/th-ch.py` to test the submission.
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
        == 54
    )
